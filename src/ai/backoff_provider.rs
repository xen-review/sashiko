// Copyright 2026 The Sashiko Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A pass-through [`AiProvider`] decorator that retries with backoff on
//! rate-limit and transient (e.g. 503 "overloaded") errors.
//!
//! The daemon classifies provider errors and backs off before retrying them.
//! A worker running reviews in-process calls the provider directly, so its only
//! recourse is the stage loop retrying immediately and blindly. This decorator
//! gives that path the same behaviour, reusing [`QuotaManager`] and the typed
//! [`AiErrorClass`] classification the providers already produce:
//!
//! - `RateLimit` (429 / quota): account-wide, so it is reported to a shared
//!   [`QuotaManager`] and every concurrent request waits out the window.
//! - `Transient` (503 overloaded, 500/502/504, 529, and transport failures
//!   such as a refused or reset connection, a timeout or a response body cut
//!   off): a momentary failure, so only this call backs off, exponentially
//!   and with jitter so concurrent stages do not resynchronise onto the same
//!   retry instant.
//! - `Fatal` (including requests that cannot be built or that hit a redirect
//!   loop): propagated immediately.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio::time::sleep;
use tracing::warn;

use crate::ai::quota::QuotaManager;
use crate::ai::{
    AiErrorClass, AiProvider, AiRequest, AiResponse, CacheStats, ProviderCapabilities,
    classify_ai_error, get_log_prefix,
};

/// Maximum number of attempts (initial try plus retries) for a single call
/// before the last error is propagated. Bounds wall-clock so a sustained
/// outage cannot hang a review indefinitely; transient blips resolve well
/// within this.
const MAX_ATTEMPTS: u32 = 6;

/// Lets a caller account for the time spent waiting on the shared quota gate
/// and stop the retry loop early.
///
/// The daemon bounds a review by an activity deadline rather than by an attempt
/// count, and does not want a rate-limit wait to consume that budget, so it
/// credits the wait back and fails once the deadline passes.
pub trait RetryBudget: Send + Sync {
    /// Time spent blocked on the shared quota gate before an attempt.
    fn credit_wait(&self, slept: Duration);
    /// Return an error to abandon further retries.
    fn check(&self) -> Result<()>;
    fn wait_started(&self) {}
    fn wait_finished(&self, slept: Duration) {
        self.credit_wait(slept);
    }
}

/// A [`RetryBudget`] backed by a wall-clock deadline shared with the caller.
///
/// Time spent parked on the quota gate is credited back, so waiting out a rate
/// limit does not consume the caller's budget.
pub struct DeadlineBudget {
    deadline: Arc<std::sync::Mutex<tokio::time::Instant>>,
    wall_deadline: Option<tokio::time::Instant>,
    last_credit_end: std::sync::Mutex<Option<tokio::time::Instant>>,
    active_waits: std::sync::Mutex<ActiveQuotaWaits>,
    wait_changes: tokio::sync::watch::Sender<()>,
}

#[derive(Default)]
struct ActiveQuotaWaits {
    count: usize,
    started_at: Option<tokio::time::Instant>,
}

impl DeadlineBudget {
    pub fn new(deadline: Arc<std::sync::Mutex<tokio::time::Instant>>) -> Self {
        let (wait_changes, _) = tokio::sync::watch::channel(());
        Self {
            deadline,
            wall_deadline: None,
            last_credit_end: std::sync::Mutex::new(None),
            active_waits: std::sync::Mutex::new(ActiveQuotaWaits::default()),
            wait_changes,
        }
    }

    #[cfg(feature = "server")]
    pub(crate) fn with_wall_deadline(
        deadline: Arc<std::sync::Mutex<tokio::time::Instant>>,
        wall_deadline: tokio::time::Instant,
    ) -> Self {
        let mut budget = Self::new(deadline);
        budget.wall_deadline = Some(wall_deadline);
        budget
    }

    #[cfg(feature = "server")]
    pub(crate) fn subscribe_quota_waits(&self) -> tokio::sync::watch::Receiver<()> {
        self.wait_changes.subscribe()
    }

    pub(crate) fn quota_wait_is_active(&self) -> bool {
        let waits = self.active_waits.lock().unwrap();
        let deadline = *self.deadline.lock().unwrap();
        waits.started_at.is_some_and(|started| started < deadline)
    }

    fn credit_wait_at(&self, slept: Duration, now: tokio::time::Instant) {
        if slept.is_zero() {
            return;
        }
        let sleep_start = now.checked_sub(slept).unwrap_or(now);
        let mut last_end = self.last_credit_end.lock().unwrap();
        let effective_start = match *last_end {
            Some(end) if end > sleep_start => end,
            _ => sleep_start,
        };
        if now > effective_start {
            let uncredited = now - effective_start;
            // Filter out sub-millisecond thread scheduling jitter between
            // concurrent tasks waking from the same rate-limit window.
            if uncredited >= Duration::from_millis(1) {
                let mut d = self.deadline.lock().unwrap();
                *d += uncredited;
                *last_end = Some(now);
            }
        }
    }

    fn check_at(&self, now: tokio::time::Instant) -> Result<()> {
        if self.wall_deadline.is_some_and(|deadline| now >= deadline) {
            return Err(anyhow::anyhow!(
                "Review tool timed out (wall time exceeded)"
            ));
        }
        let current = *self.deadline.lock().unwrap();
        if now > current && !self.quota_wait_is_active() && now > *self.deadline.lock().unwrap() {
            return Err(anyhow::anyhow!(
                "Review tool timed out (active time exceeded)"
            ));
        }
        Ok(())
    }
}

impl RetryBudget for DeadlineBudget {
    fn credit_wait(&self, slept: Duration) {
        self.credit_wait_at(slept, tokio::time::Instant::now());
    }

    fn check(&self) -> Result<()> {
        self.check_at(tokio::time::Instant::now())
    }

    fn wait_started(&self) {
        let mut waits = self.active_waits.lock().unwrap();
        if waits.count == 0 {
            waits.started_at = Some(tokio::time::Instant::now());
        }
        waits.count += 1;
        drop(waits);
        self.wait_changes.send_replace(());
    }

    fn wait_finished(&self, slept: Duration) {
        self.credit_wait(slept);
        let mut waits = self.active_waits.lock().unwrap();
        waits.count -= 1;
        if waits.count == 0 {
            waits.started_at = None;
        }
        drop(waits);
        self.wait_changes.send_replace(());
    }
}

struct BudgetWait<'a> {
    budget: Option<&'a dyn RetryBudget>,
    started_at: Option<tokio::time::Instant>,
    slept: Option<Duration>,
}

impl<'a> BudgetWait<'a> {
    fn new(budget: Option<&'a dyn RetryBudget>) -> Self {
        Self {
            budget,
            started_at: None,
            slept: None,
        }
    }

    fn start(&mut self) {
        if self.started_at.is_none()
            && let Some(budget) = self.budget
        {
            self.started_at = Some(tokio::time::Instant::now());
            budget.wait_started();
        }
    }

    fn finish(mut self, slept: Duration) {
        self.slept = Some(slept);
    }
}

impl Drop for BudgetWait<'_> {
    fn drop(&mut self) {
        if let Some(budget) = self.budget {
            if let Some(started_at) = self.started_at {
                budget.wait_finished(self.slept.unwrap_or_else(|| started_at.elapsed()));
            } else if let Some(slept) = self.slept {
                budget.credit_wait(slept);
            }
        }
    }
}

/// Adds rate-limit and transient retry with backoff around an inner provider.
pub struct BackoffProvider {
    inner: Arc<dyn AiProvider>,
    /// Shared across the run, so one rate-limit response backs every
    /// concurrent request off together.
    quota: Arc<QuotaManager>,
    /// Base unit of the exponential transient backoff. Tests use a tiny value
    /// so they run in real time.
    base_delay: Duration,
    /// Attempt ceiling, or None to retry until `budget` says to stop.
    max_attempts: Option<u32>,
    budget: Option<Arc<dyn RetryBudget>>,
}

impl BackoffProvider {
    /// With a `budget`, retries until it reports the caller's deadline has
    /// passed. Without one, falls back to a fixed attempt ceiling, which bounds
    /// wall-clock for callers that have no deadline of their own.
    pub fn new(
        inner: Arc<dyn AiProvider>,
        quota: Arc<QuotaManager>,
        budget: Option<Arc<dyn RetryBudget>>,
    ) -> Self {
        Self {
            inner,
            quota,
            base_delay: Duration::from_secs(1),
            max_attempts: budget.is_none().then_some(MAX_ATTEMPTS),
            budget,
        }
    }
}

#[async_trait]
impl AiProvider for BackoffProvider {
    async fn generate_content(&self, request: AiRequest) -> Result<AiResponse> {
        let mut attempt: u32 = 0;
        let mut transient_streak: i32 = 0;
        loop {
            // Honour any active global rate-limit window before trying. The
            // wait is reported so a caller can keep it off its own deadline.
            let mut wait = BudgetWait::new(self.budget.as_deref());
            let slept = self.quota.wait_for_access_with(|| wait.start()).await;
            wait.finish(slept);
            if let Some(budget) = &self.budget {
                budget.check()?;
            }

            match self.inner.generate_content(request.clone()).await {
                Ok(response) => {
                    self.quota.report_success().await;
                    return Ok(response);
                }
                Err(e) => {
                    attempt += 1;
                    let classification = classify_ai_error(&e);
                    if let AiErrorClass::RateLimit { retry_after } = &classification {
                        self.quota.report_quota_error(*retry_after).await;
                    }
                    if let Some(max) = self.max_attempts
                        && attempt >= max
                    {
                        return Err(e);
                    }
                    match classification {
                        AiErrorClass::RateLimit { .. } => {}
                        AiErrorClass::Transient { retry_after } => {
                            // Server-side blip. Exponential backoff with
                            // jitter, floored by any server-suggested delay.
                            // Per-call, not global.
                            transient_streak += 1;
                            let mult = 2.0_f64.powi(transient_streak - 1).min(60.0);
                            let backoff = self.base_delay.mul_f64(mult).max(retry_after);
                            let jittered = backoff + backoff.mul_f64(0.25 * fastrand::f64());
                            let retry_target = match self.max_attempts {
                                Some(max) => format!("{attempt}/{max}"),
                                None => format!("{attempt}"),
                            };
                            warn!(
                                "{}Transient AI error (streak {}); backing off {:.1}s then retry {}: {}",
                                get_log_prefix(),
                                transient_streak,
                                jittered.as_secs_f64(),
                                retry_target,
                                e
                            );
                            sleep(jittered).await;
                        }
                        AiErrorClass::Fatal => return Err(e),
                    }
                }
            }
        }
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        self.inner.get_capabilities()
    }

    fn cache_identity(&self) -> String {
        self.inner.cache_identity()
    }

    fn cache_stats(&self) -> Option<CacheStats> {
        self.inner.cache_stats()
    }

    async fn forget(&self, request: &AiRequest) {
        self.inner.forget(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::gemini::GeminiError;
    use crate::ai::{AiMessage, AiRole};
    use std::sync::atomic::{AtomicU32, Ordering};

    enum Behaviour {
        Transient,
        RateLimit,
        Fatal,
    }

    struct MockProvider {
        calls: AtomicU32,
        fail_times: u32,
        behaviour: Behaviour,
        rate_limit_after: Duration,
    }

    #[async_trait]
    impl AiProvider for MockProvider {
        async fn generate_content(&self, _request: AiRequest) -> Result<AiResponse> {
            let n = self.calls.fetch_add(1, Ordering::SeqCst);
            if n < self.fail_times {
                let err = match self.behaviour {
                    Behaviour::Transient => {
                        GeminiError::TransientError(Duration::from_secs(0), "503 overloaded".into())
                    }
                    Behaviour::RateLimit => GeminiError::QuotaExceeded(self.rate_limit_after),
                    Behaviour::Fatal => GeminiError::PermissionDenied("nope".into()),
                };
                return Err(err.into());
            }
            Ok(AiResponse {
                content: Some("ok".into()),
                thought: None,
                thought_signature: None,
                tool_calls: None,
                usage: None,
                truncated: false,
            })
        }

        fn get_capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities {
                model_name: "mock".into(),
                context_window_size: 1000,
            }
        }
    }

    fn mock(
        behaviour: Behaviour,
        fail_times: u32,
        rate_limit_after: Duration,
    ) -> Arc<MockProvider> {
        Arc::new(MockProvider {
            calls: AtomicU32::new(0),
            fail_times,
            behaviour,
            rate_limit_after,
        })
    }

    /// A BackoffProvider with a tiny base delay so transient backoff runs in
    /// real time without needing the tokio virtual clock.
    fn fast(inner: Arc<dyn AiProvider>) -> BackoffProvider {
        BackoffProvider {
            inner,
            quota: Arc::new(QuotaManager::new()),
            base_delay: Duration::from_millis(1),
            max_attempts: Some(MAX_ATTEMPTS),
            budget: None,
        }
    }

    fn dummy_request() -> AiRequest {
        AiRequest {
            system: None,
            messages: vec![AiMessage {
                role: AiRole::User,
                content: Some("hi".into()),
                thought: None,
                thought_signature: None,
                tool_calls: None,
                tool_call_id: None,
            }],
            tools: None,
            temperature: None,
            response_format: None,
            context_tag: None,
            reasoning_effort: None,
        }
    }

    #[tokio::test]
    async fn transient_backs_off_then_succeeds() {
        let m = mock(Behaviour::Transient, 3, Duration::ZERO);
        let resp = fast(m.clone())
            .generate_content(dummy_request())
            .await
            .unwrap();
        assert_eq!(resp.content.as_deref(), Some("ok"));
        assert_eq!(m.calls.load(Ordering::SeqCst), 4); // 3 transient failures + success
    }

    #[tokio::test]
    async fn transient_gives_up_after_max_attempts() {
        let m = mock(Behaviour::Transient, u32::MAX, Duration::ZERO);
        let err = fast(m.clone()).generate_content(dummy_request()).await;
        assert!(err.is_err());
        assert_eq!(m.calls.load(Ordering::SeqCst), MAX_ATTEMPTS); // bounded
    }

    #[tokio::test]
    async fn fatal_propagates_without_retry() {
        let m = mock(Behaviour::Fatal, u32::MAX, Duration::ZERO);
        let err = fast(m.clone()).generate_content(dummy_request()).await;
        assert!(err.is_err());
        assert_eq!(m.calls.load(Ordering::SeqCst), 1); // no retry on fatal
    }

    #[tokio::test]
    async fn expired_budget_stops_before_calling() {
        struct Expired;
        impl RetryBudget for Expired {
            fn credit_wait(&self, _slept: Duration) {}
            fn check(&self) -> Result<()> {
                Err(anyhow::anyhow!("deadline exceeded"))
            }
        }
        let m = mock(Behaviour::Transient, u32::MAX, Duration::ZERO);
        let provider = BackoffProvider::new(
            m.clone(),
            Arc::new(QuotaManager::new()),
            Some(Arc::new(Expired)),
        );
        assert!(provider.generate_content(dummy_request()).await.is_err());
        // The budget gates the loop, so the inner provider is never reached.
        assert_eq!(m.calls.load(Ordering::SeqCst), 0);
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn expired_wall_deadline_stops_before_calling() {
        let now = tokio::time::Instant::now();
        let deadline = Arc::new(std::sync::Mutex::new(now + Duration::from_secs(60)));
        let budget = Arc::new(DeadlineBudget::with_wall_deadline(deadline, now));
        let inner = mock(Behaviour::Transient, u32::MAX, Duration::ZERO);
        let provider =
            BackoffProvider::new(inner.clone(), Arc::new(QuotaManager::new()), Some(budget));

        let error = provider
            .generate_content(dummy_request())
            .await
            .unwrap_err();
        assert!(error.to_string().contains("wall time exceeded"));
        assert_eq!(inner.calls.load(Ordering::SeqCst), 0);
    }

    #[cfg(feature = "server")]
    async fn queued_call_stops_after_budget_expires(message: &'static str) {
        struct SignalBudget {
            checked: tokio::sync::Notify,
            expired: std::sync::atomic::AtomicBool,
            message: &'static str,
        }
        impl RetryBudget for SignalBudget {
            fn credit_wait(&self, _slept: Duration) {}
            fn check(&self) -> Result<()> {
                if self.expired.load(Ordering::SeqCst) {
                    return Err(anyhow::anyhow!(self.message));
                }
                self.checked.notify_one();
                Ok(())
            }
        }

        let semaphore = Arc::new(tokio::sync::Semaphore::new(1));
        let permit = semaphore.acquire().await.unwrap();
        let inner = mock(Behaviour::Transient, 0, Duration::ZERO);
        let budget = Arc::new(SignalBudget {
            checked: tokio::sync::Notify::new(),
            expired: std::sync::atomic::AtomicBool::new(false),
            message,
        });
        let limited = Arc::new(
            crate::ai::concurrency_limited_provider::ConcurrencyLimitedProvider::with_retry_budget(
                inner.clone(),
                semaphore.clone(),
                budget.clone(),
            ),
        );
        let provider =
            BackoffProvider::new(limited, Arc::new(QuotaManager::new()), Some(budget.clone()));
        let call = tokio::spawn(async move { provider.generate_content(dummy_request()).await });
        tokio::time::timeout(Duration::from_secs(2), budget.checked.notified())
            .await
            .unwrap();
        budget.expired.store(true, Ordering::SeqCst);
        drop(permit);

        let error = call.await.unwrap().unwrap_err();
        assert_eq!(error.to_string(), message);
        assert_eq!(inner.calls.load(Ordering::SeqCst), 0);
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn queued_call_does_not_start_after_wall_deadline() {
        queued_call_stops_after_budget_expires("Review tool timed out (wall time exceeded)").await;
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn queued_call_does_not_start_after_active_deadline() {
        queued_call_stops_after_budget_expires("Review tool timed out (active time exceeded)")
            .await;
    }

    #[tokio::test]
    async fn rate_limit_waits_then_succeeds() {
        // QuotaManager uses std::Instant, so use a tiny real delay here.
        let m = mock(Behaviour::RateLimit, 2, Duration::from_millis(5));
        let resp = fast(m.clone())
            .generate_content(dummy_request())
            .await
            .unwrap();
        assert_eq!(resp.content.as_deref(), Some("ok"));
        assert_eq!(m.calls.load(Ordering::SeqCst), 3); // 2 rate-limit failures + success
    }

    #[tokio::test]
    async fn terminal_rate_limit_blocks_other_requests() {
        let inner = mock(Behaviour::RateLimit, 1, Duration::from_secs(60));
        let quota = Arc::new(QuotaManager::new());
        let provider = BackoffProvider {
            inner: inner.clone(),
            quota: quota.clone(),
            base_delay: Duration::from_millis(1),
            max_attempts: Some(1),
            budget: None,
        };

        assert!(provider.generate_content(dummy_request()).await.is_err());
        assert_eq!(inner.calls.load(Ordering::SeqCst), 1);
        assert!(
            tokio::time::timeout(Duration::from_millis(20), quota.wait_for_access())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn budget_wait_guards_release_after_overlap_and_cancellation() {
        let deadline = Arc::new(std::sync::Mutex::new(
            tokio::time::Instant::now() + Duration::from_secs(60),
        ));
        let budget = DeadlineBudget::new(deadline);
        let changes = budget.wait_changes.subscribe();
        let mut first = BudgetWait::new(Some(&budget));
        let mut second = BudgetWait::new(Some(&budget));

        first.start();
        second.start();
        assert_eq!(budget.active_waits.lock().unwrap().count, 2);
        first.finish(Duration::from_millis(5));
        assert_eq!(budget.active_waits.lock().unwrap().count, 1);
        drop(second);
        assert_eq!(budget.active_waits.lock().unwrap().count, 0);
        assert!(budget.active_waits.lock().unwrap().started_at.is_none());
        assert!(changes.has_changed().unwrap());
    }

    #[cfg(feature = "server")]
    #[tokio::test]
    async fn quota_wait_started_after_deadline_does_not_pause_it() {
        let deadline = Arc::new(std::sync::Mutex::new(tokio::time::Instant::now()));
        let budget = DeadlineBudget::new(deadline);
        let mut wait = BudgetWait::new(Some(&budget));

        wait.start();
        assert!(!budget.quota_wait_is_active());
        assert!(budget.check().is_err());
    }

    #[cfg(feature = "server")]
    #[test]
    fn active_quota_wait_pauses_active_deadline_but_not_wall_deadline() {
        let now = tokio::time::Instant::now();
        let deadline = Arc::new(std::sync::Mutex::new(now + Duration::from_secs(60)));
        let wall_deadline = now + Duration::from_secs(180);
        let budget = DeadlineBudget::with_wall_deadline(deadline, wall_deadline);
        let mut wait = BudgetWait::new(Some(&budget));

        wait.start();
        assert!(budget.check_at(now + Duration::from_secs(120)).is_ok());
        let error = budget.check_at(wall_deadline).unwrap_err();
        assert!(error.to_string().contains("wall time exceeded"));

        drop(wait);
        let error = budget.check_at(now + Duration::from_secs(120)).unwrap_err();
        assert!(error.to_string().contains("active time exceeded"));
    }

    #[tokio::test]
    async fn deadline_budget_credit_wait_deduplicates_overlapping_sleeps() {
        let now = tokio::time::Instant::now() + Duration::from_secs(30);
        let deadline = Arc::new(std::sync::Mutex::new(now + Duration::from_secs(60)));
        let budget = DeadlineBudget::new(deadline.clone());

        budget.credit_wait_at(Duration::from_secs(10), now);
        let first_credit = *deadline.lock().unwrap();
        assert_eq!(first_credit, now + Duration::from_secs(70));

        budget.credit_wait_at(Duration::from_secs(10), now);
        let repeated_credit = *deadline.lock().unwrap();
        assert_eq!(first_credit, repeated_credit);

        budget.credit_wait_at(Duration::from_secs(10), now + Duration::from_secs(5));
        let overlap_credit = *deadline.lock().unwrap();
        assert_eq!(overlap_credit, repeated_credit + Duration::from_secs(5));

        budget.credit_wait_at(Duration::from_secs(10), now + Duration::from_secs(20));
        let separate_credit = *deadline.lock().unwrap();
        assert_eq!(separate_credit, overlap_credit + Duration::from_secs(10));

        budget.credit_wait_at(Duration::ZERO, now + Duration::from_secs(20));
        assert_eq!(*deadline.lock().unwrap(), separate_credit);
    }
}

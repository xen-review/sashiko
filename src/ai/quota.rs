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

use anyhow::{Result, ensure};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{info, warn};

pub(crate) const MAX_RETRY_AFTER: Duration = Duration::from_secs(5 * 60);

pub struct QuotaManager {
    // Stores the time when we can resume making requests.
    // If None or in the past, we are free to go.
    blocked_until: Mutex<Option<Instant>>,
}

impl Default for QuotaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl QuotaManager {
    pub fn new() -> Self {
        Self {
            blocked_until: Mutex::new(None),
        }
    }

    pub async fn wait_for_access(&self) -> Duration {
        self.wait_for_access_with(|| {}).await
    }

    pub(crate) async fn is_blocked(&self) -> bool {
        self.blocked_until
            .lock()
            .await
            .is_some_and(|until| until > Instant::now())
    }

    pub(crate) async fn wait_for_access_with<F>(&self, mut on_wait: F) -> Duration
    where
        F: FnMut(),
    {
        let mut total_slept = Duration::ZERO;
        loop {
            let sleep_duration = {
                let guard = self.blocked_until.lock().await;
                if let Some(until) = *guard {
                    let now = Instant::now();
                    if until > now { Some(until - now) } else { None }
                } else {
                    None
                }
            };

            if let Some(duration) = sleep_duration {
                on_wait();
                info!(
                    "{}Global AI rate limit/quota active. Waiting for {:.2}s...",
                    crate::ai::get_log_prefix(),
                    duration.as_secs_f64()
                );
                tokio::time::sleep(duration).await;
                total_slept += duration;
            } else {
                break;
            }
        }
        total_slept
    }

    pub async fn report_success(&self) {
        let mut guard = self.blocked_until.lock().await;
        if guard.is_some_and(|until| until <= Instant::now()) {
            *guard = None;
            info!("AI request succeeded, resetting quota backoff.");
        }
    }

    pub async fn report_quota_error(&self, retry_after: Duration) {
        let retry_after = retry_after.min(MAX_RETRY_AFTER);
        let mut guard = self.blocked_until.lock().await;
        let resume_time = Instant::now() + retry_after;

        if let Some(current) = *guard {
            if resume_time > current {
                *guard = Some(resume_time);
            }
        } else {
            *guard = Some(resume_time);
        }

        warn!(
            "Quota exhausted! Blocking all LLM requests for {:.2}s",
            retry_after.as_secs_f64()
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RemainingQuotaPercent(f64);

impl RemainingQuotaPercent {
    pub fn new(value: f64) -> Result<Self> {
        ensure!(
            value.is_finite() && (0.0..=100.0).contains(&value),
            "Invalid remaining quota percentage"
        );
        Ok(Self(value))
    }

    pub(crate) fn exceeds(self, minimum: f64) -> bool {
        self.0 > minimum
    }

    #[cfg(feature = "server")]
    pub(crate) fn as_f64(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct FiveHourQuotaSample {
    pub account_key: String,
    pub observed_at: i64,
    pub remaining: RemainingQuotaPercent,
    pub resets_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountQuotaPauseReason {
    LowAllowance,
    AwaitingResetEvidence,
    TelemetryUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountQuotaAdmission {
    Ready,
    Paused {
        reason: AccountQuotaPauseReason,
        resume_not_before: Option<i64>,
        next_check_at: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountQuotaState {
    pub account_key: String,
    pub admission: AccountQuotaAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotaAdmissionGate {
    Bypassed,
    FiveHourReady,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotaAdmissionDecision {
    pub gate: QuotaAdmissionGate,
    pub state: AccountQuotaState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CodexAccountQuotaPolicy {
    pub enabled: bool,
    pub pause_below_percent: f64,
    pub resume_above_percent: f64,
    pub max_sample_age_seconds: i64,
    pub unavailable_retry_seconds: i64,
}

impl Default for CodexAccountQuotaPolicy {
    fn default() -> Self {
        Self {
            enabled: false,
            pause_below_percent: 20.0,
            resume_above_percent: 30.0,
            max_sample_age_seconds: 300,
            unavailable_retry_seconds: 60,
        }
    }
}

impl CodexAccountQuotaPolicy {
    pub fn evaluate(
        &self,
        now: i64,
        state: &AccountQuotaState,
        sample: Option<&FiveHourQuotaSample>,
    ) -> Result<QuotaAdmissionDecision> {
        if !self.enabled {
            return Ok(QuotaAdmissionDecision {
                gate: QuotaAdmissionGate::Bypassed,
                state: state.clone(),
            });
        }
        self.validate()?;
        ensure!(
            now >= 0 && !state.account_key.trim().is_empty(),
            "Invalid quota clock or account identity"
        );
        let barrier = match state.admission {
            AccountQuotaAdmission::Ready => None,
            AccountQuotaAdmission::Paused {
                resume_not_before, ..
            } => resume_not_before,
        };
        let sample = self.usable_sample(now, &state.account_key, sample);
        let Some(sample) = sample else {
            return self.pause(
                now,
                state,
                AccountQuotaPauseReason::TelemetryUnavailable,
                barrier,
            );
        };
        let was_paused = matches!(state.admission, AccountQuotaAdmission::Paused { .. });
        if sample.remaining.0 < self.pause_below_percent
            || (was_paused && sample.remaining.0 <= self.resume_above_percent)
        {
            let barrier =
                Some(barrier.map_or(sample.resets_at, |previous| previous.max(sample.resets_at)));
            return self.pause(now, state, AccountQuotaPauseReason::LowAllowance, barrier);
        }
        let barrier = barrier.map(|previous| {
            if sample.observed_at < previous {
                previous.max(sample.resets_at)
            } else {
                previous
            }
        });
        if let Some(barrier) = barrier
            && (now < barrier || sample.observed_at < barrier)
        {
            return self.pause(
                now,
                state,
                AccountQuotaPauseReason::AwaitingResetEvidence,
                Some(barrier),
            );
        }
        Ok(QuotaAdmissionDecision {
            gate: QuotaAdmissionGate::FiveHourReady,
            state: AccountQuotaState {
                account_key: state.account_key.clone(),
                admission: AccountQuotaAdmission::Ready,
            },
        })
    }

    pub(crate) fn usable_sample<'a>(
        &self,
        now: i64,
        account_key: &str,
        sample: Option<&'a FiveHourQuotaSample>,
    ) -> Option<&'a FiveHourQuotaSample> {
        sample.filter(|sample| {
            sample.account_key == account_key
                && sample.observed_at >= 0
                && sample.observed_at <= now
                && now - sample.observed_at <= self.max_sample_age_seconds
                && sample.resets_at > now
        })
    }

    pub(crate) fn validate(&self) -> Result<()> {
        ensure!(
            self.pause_below_percent.is_finite()
                && self.resume_above_percent.is_finite()
                && (0.0..=100.0).contains(&self.pause_below_percent)
                && (0.0..=100.0).contains(&self.resume_above_percent)
                && self.pause_below_percent < self.resume_above_percent,
            "Quota thresholds must be finite, ordered percentages in 0..100"
        );
        ensure!(
            (1..=3600).contains(&self.max_sample_age_seconds)
                && (1..=3600).contains(&self.unavailable_retry_seconds),
            "Quota sample age and retry durations must be in 1..3600 seconds"
        );
        Ok(())
    }

    fn pause(
        &self,
        now: i64,
        state: &AccountQuotaState,
        reason: AccountQuotaPauseReason,
        barrier: Option<i64>,
    ) -> Result<QuotaAdmissionDecision> {
        let next_check_at = match barrier.filter(|barrier| *barrier > now) {
            Some(barrier) => barrier,
            None => now
                .checked_add(self.unavailable_retry_seconds)
                .ok_or_else(|| anyhow::anyhow!("Quota retry timestamp overflows"))?,
        };
        Ok(QuotaAdmissionDecision {
            gate: QuotaAdmissionGate::Paused,
            state: AccountQuotaState {
                account_key: state.account_key.clone(),
                admission: AccountQuotaAdmission::Paused {
                    reason,
                    resume_not_before: barrier,
                    next_check_at,
                },
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn concurrent_success_does_not_clear_active_backoff() {
        let quota = QuotaManager::new();
        assert!(!quota.is_blocked().await);
        quota.report_quota_error(Duration::from_secs(60)).await;
        quota.report_success().await;

        assert!(quota.is_blocked().await);
        assert!(
            tokio::time::timeout(Duration::from_millis(20), quota.wait_for_access())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn success_clears_expired_backoff() {
        let quota = QuotaManager::new();
        quota.report_quota_error(Duration::ZERO).await;
        assert!(!quota.is_blocked().await);
        quota.report_success().await;

        assert!(quota.blocked_until.lock().await.is_none());
        assert_eq!(quota.wait_for_access().await, Duration::ZERO);
    }

    fn ready() -> AccountQuotaState {
        AccountQuotaState {
            account_key: "account-a".into(),
            admission: AccountQuotaAdmission::Ready,
        }
    }

    fn sample(remaining: f64, observed_at: i64, resets_at: i64) -> FiveHourQuotaSample {
        FiveHourQuotaSample {
            account_key: "account-a".into(),
            observed_at,
            remaining: RemainingQuotaPercent::new(remaining).unwrap(),
            resets_at,
        }
    }

    fn policy() -> CodexAccountQuotaPolicy {
        CodexAccountQuotaPolicy {
            enabled: true,
            ..Default::default()
        }
    }

    #[test]
    fn account_policy_has_strict_pause_and_resume_thresholds() -> Result<()> {
        let policy = policy();
        for (remaining, gate) in [
            (19.9, QuotaAdmissionGate::Paused),
            (20.0, QuotaAdmissionGate::FiveHourReady),
            (20.1, QuotaAdmissionGate::FiveHourReady),
        ] {
            assert_eq!(
                policy
                    .evaluate(10, &ready(), Some(&sample(remaining, 10, 100)))?
                    .gate,
                gate
            );
        }
        let paused = policy
            .evaluate(10, &ready(), Some(&sample(19.0, 10, 100)))?
            .state;
        for (remaining, gate) in [
            (29.9, QuotaAdmissionGate::Paused),
            (30.0, QuotaAdmissionGate::Paused),
            (30.1, QuotaAdmissionGate::FiveHourReady),
        ] {
            assert_eq!(
                policy
                    .evaluate(100, &paused, Some(&sample(remaining, 100, 200)))?
                    .gate,
                gate
            );
        }
        Ok(())
    }

    #[test]
    fn account_policy_requires_post_reset_observation_and_retains_disabled_state() -> Result<()> {
        let policy = policy();
        let paused = policy
            .evaluate(10, &ready(), Some(&sample(10.0, 10, 100)))?
            .state;
        assert_eq!(
            policy
                .evaluate(99, &paused, Some(&sample(100.0, 99, 200)))?
                .gate,
            QuotaAdmissionGate::Paused
        );
        assert_eq!(
            policy
                .evaluate(100, &paused, Some(&sample(100.0, 99, 200)))?
                .gate,
            QuotaAdmissionGate::Paused
        );
        assert_eq!(
            policy
                .evaluate(100, &paused, Some(&sample(100.0, 100, 200)))?
                .gate,
            QuotaAdmissionGate::FiveHourReady
        );
        let bypass = CodexAccountQuotaPolicy::default().evaluate(100, &paused, None)?;
        assert_eq!(bypass.gate, QuotaAdmissionGate::Bypassed);
        assert_eq!(bypass.state, paused);
        Ok(())
    }

    #[test]
    fn account_policy_rejects_unusable_readings_without_crossing_accounts() -> Result<()> {
        let policy = policy();
        let paused = policy
            .evaluate(10, &ready(), Some(&sample(10.0, 10, 1000)))?
            .state;
        for reading in [
            None,
            Some(sample(80.0, 199, 1000)),
            Some(sample(80.0, 501, 1000)),
            Some(sample(80.0, 500, 500)),
            Some(sample(80.0, -1, 1000)),
            Some(FiveHourQuotaSample {
                account_key: "account-b".into(),
                ..sample(80.0, 500, 2000)
            }),
        ] {
            let blocked = policy.evaluate(500, &ready(), reading.as_ref())?;
            assert_eq!(blocked.state.account_key, "account-a");
            assert_eq!(
                blocked.state.admission,
                AccountQuotaAdmission::Paused {
                    reason: AccountQuotaPauseReason::TelemetryUnavailable,
                    resume_not_before: None,
                    next_check_at: 560,
                }
            );
            assert_eq!(
                policy
                    .evaluate(500, &paused, reading.as_ref())?
                    .state
                    .admission,
                AccountQuotaAdmission::Paused {
                    reason: AccountQuotaPauseReason::TelemetryUnavailable,
                    resume_not_before: Some(1000),
                    next_check_at: 1000
                }
            );
        }
        assert_eq!(
            policy
                .evaluate(500, &ready(), Some(&sample(80.0, 200, 1000)))?
                .gate,
            QuotaAdmissionGate::FiveHourReady
        );
        Ok(())
    }

    #[test]
    fn account_policy_retains_shifted_reset_barriers_and_handles_late_wakeups() -> Result<()> {
        let policy = policy();
        let paused = policy
            .evaluate(10, &ready(), Some(&sample(10.0, 10, 100)))?
            .state;
        let earlier = policy
            .evaluate(20, &paused, Some(&sample(15.0, 20, 90)))?
            .state;
        assert_eq!(
            earlier.admission,
            AccountQuotaAdmission::Paused {
                reason: AccountQuotaPauseReason::LowAllowance,
                resume_not_before: Some(100),
                next_check_at: 100,
            }
        );
        let later = policy
            .evaluate(25, &earlier, Some(&sample(15.0, 25, 200)))?
            .state;
        assert_eq!(
            later.admission,
            AccountQuotaAdmission::Paused {
                reason: AccountQuotaPauseReason::LowAllowance,
                resume_not_before: Some(200),
                next_check_at: 200,
            }
        );
        let restored = later.clone();
        assert_eq!(
            policy
                .evaluate(100, &restored, Some(&sample(80.0, 100, 300)))?
                .gate,
            QuotaAdmissionGate::Paused
        );
        assert_eq!(
            policy
                .evaluate(500, &restored, Some(&sample(80.0, 500, 600)))?
                .gate,
            QuotaAdmissionGate::FiveHourReady
        );
        assert_eq!(
            policy
                .evaluate(500, &restored, Some(&sample(29.0, 500, 600)))?
                .state
                .admission,
            AccountQuotaAdmission::Paused {
                reason: AccountQuotaPauseReason::LowAllowance,
                resume_not_before: Some(600),
                next_check_at: 600
            }
        );
        assert_eq!(
            policy.evaluate(500, &restored, None)?.state.admission,
            AccountQuotaAdmission::Paused {
                reason: AccountQuotaPauseReason::TelemetryUnavailable,
                resume_not_before: Some(200),
                next_check_at: 560
            }
        );
        Ok(())
    }

    #[test]
    fn account_policy_retains_later_reset_reported_before_barrier() -> Result<()> {
        let policy = policy();
        let paused = policy
            .evaluate(10, &ready(), Some(&sample(10.0, 10, 100)))?
            .state;
        let earlier = policy
            .evaluate(50, &paused, Some(&sample(80.0, 50, 90)))?
            .state;
        assert_eq!(
            earlier.admission,
            AccountQuotaAdmission::Paused {
                reason: AccountQuotaPauseReason::AwaitingResetEvidence,
                resume_not_before: Some(100),
                next_check_at: 100,
            }
        );
        let shifted = policy
            .evaluate(90, &paused, Some(&sample(80.0, 90, 200)))?
            .state;
        assert_eq!(
            shifted.admission,
            AccountQuotaAdmission::Paused {
                reason: AccountQuotaPauseReason::AwaitingResetEvidence,
                resume_not_before: Some(200),
                next_check_at: 200,
            }
        );
        let late = policy
            .evaluate(101, &paused, Some(&sample(80.0, 90, 200)))?
            .state;
        assert_eq!(late.admission, shifted.admission);
        assert_eq!(
            policy
                .evaluate(100, &shifted, Some(&sample(80.0, 100, 200)))?
                .gate,
            QuotaAdmissionGate::Paused
        );
        assert_eq!(
            policy
                .evaluate(200, &shifted, Some(&sample(80.0, 200, 300)))?
                .gate,
            QuotaAdmissionGate::FiveHourReady
        );
        Ok(())
    }

    #[test]
    fn account_policy_validates_inputs_and_checks_timestamp_overflow() -> Result<()> {
        for invalid in [f64::NAN, f64::INFINITY, -0.1, 100.1] {
            assert!(RemainingQuotaPercent::new(invalid).is_err());
        }
        for invalid in [
            CodexAccountQuotaPolicy {
                pause_below_percent: f64::NAN,
                ..policy()
            },
            CodexAccountQuotaPolicy {
                pause_below_percent: 31.0,
                ..policy()
            },
            CodexAccountQuotaPolicy {
                resume_above_percent: 20.0,
                ..policy()
            },
            CodexAccountQuotaPolicy {
                max_sample_age_seconds: 0,
                ..policy()
            },
            CodexAccountQuotaPolicy {
                unavailable_retry_seconds: 3601,
                ..policy()
            },
        ] {
            assert!(invalid.evaluate(500, &ready(), None).is_err());
            let disabled = CodexAccountQuotaPolicy {
                enabled: false,
                ..invalid
            };
            assert_eq!(
                disabled.evaluate(i64::MAX, &ready(), None)?.gate,
                QuotaAdmissionGate::Bypassed
            );
        }
        assert!(policy().evaluate(-1, &ready(), None).is_err());
        assert!(
            policy()
                .evaluate(
                    0,
                    &AccountQuotaState {
                        account_key: " ".into(),
                        admission: AccountQuotaAdmission::Ready
                    },
                    None
                )
                .is_err()
        );
        assert!(policy().evaluate(i64::MAX, &ready(), None).is_err());
        assert_eq!(
            policy()
                .evaluate(
                    i64::MAX - 1,
                    &ready(),
                    Some(&sample(10.0, i64::MAX - 1, i64::MAX))
                )?
                .state
                .admission,
            AccountQuotaAdmission::Paused {
                reason: AccountQuotaPauseReason::LowAllowance,
                resume_not_before: Some(i64::MAX),
                next_check_at: i64::MAX
            }
        );
        Ok(())
    }
}

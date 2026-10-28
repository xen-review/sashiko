const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const html = fs.readFileSync(path.join(__dirname, '../static/index.html'), 'utf8');
const start = html.indexOf('function renderReviewTokens(');
const end = html.indexOf('function renderActiveHarnessUsage(', start);
assert(start >= 0 && end > start, 'review token renderer not found');

const { render, renderRetries } = vm.runInNewContext(
    `(function() { ${html.slice(start, end)}; return { render: renderReviewTokens, renderRetries: renderStageRetryUsage }; })()`,
    { formatTokenCount: value => String(value) },
);

assert.equal(renderRetries(undefined), '');
assert.equal(renderRetries([]), '');
const retries = renderRetries([
    { request: 1, usage: { prompt_tokens: 12, completion_tokens: 3, cached_tokens: 0 } },
    { request: 2, usage: { prompt_tokens: 7, completion_tokens: 2 } },
    { request: '<img src=x>', usage: { prompt_tokens: '<script>', completion_tokens: 1, cached_tokens: 0 } },
]);
assert.match(retries, /Session request 1: Input 12.*Cached 0.*Output 3/);
assert.match(retries, /Session request 2: Input 7.*Cached Unknown.*Output 2/);
assert.match(retries, /Session request Unknown: Input Unknown/);
assert.doesNotMatch(retries, /<img|<script>/);
assert.match(retries, /separate from stage totals/i);
assert.match(renderRetries(Array(33).fill({ request: 1, usage: {} })), /truncated/i);
assert.equal(renderRetries({ request: 1 }), '');
const invalidRetry = renderRetries([
    { request: 0, usage: { prompt_tokens: 5, completion_tokens: -1, cached_tokens: 6 } },
    { request: Number.MAX_SAFE_INTEGER + 1, usage: { prompt_tokens: 1.5, completion_tokens: 0 } },
]);
assert.match(invalidRetry, /Session request Unknown: Input 5.*Cached Unknown.*Output Unknown/);
assert.match(invalidRetry, /Session request Unknown: Input Unknown.*Cached Unknown.*Output 0/);

const record = { tokens_in: 100, tokens_cached: 0, tokens_out: 4 };
const coverage = {
    requests: 2,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
};

for (const missingCoverage of [undefined, null, { requests: 1 }]) {
    const result = render(record, missingCoverage);
    assert.match(result, /In: Unknown/);
    assert.match(result, /Cached: Unknown/);
    assert.match(result, /Out: 4 \(partial\)/);
    assert.match(result, /Usage coverage: Unknown/);
}

const unknownPartial = render({ ...record, tokens_cached: 20, tokens_out: 0 }, null);
assert.match(unknownPartial, /In: Unknown/);
assert.match(unknownPartial, /Cached: 20 \(partial\)/);
assert.match(unknownPartial, /Out: Unknown/);

for (const responsesWithoutUsage of [0, 1]) {
    const result = render(record, {
        ...coverage,
        responses_without_usage: responsesWithoutUsage,
    });
    assert.match(result, /In: Unknown/);
    assert.match(result, /Cached: Unknown/);
    assert.match(result, /Out: 4 \(partial\)/);
    assert.match(result, /incomplete/);
}

const partial = render({ ...record, tokens_cached: 20 }, {
    ...coverage,
    responses_without_usage: 1,
});
assert.match(partial, /In: Unknown/);
assert.match(partial, /Cached: 20 \(partial\)/);
assert.match(partial, /Out: 4 \(partial\)/);

const incompleteZero = render({ ...record, tokens_out: 0 }, {
    ...coverage,
    responses_without_usage: 1,
});
assert.match(incompleteZero, /Out: Unknown/);

const missingCache = render(record, {
    requests: 1,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 1,
});
assert.match(missingCache, /In: Unknown/);
assert.match(missingCache, /Cached: Unknown/);
assert.match(missingCache, /Out: 4/);
assert.doesNotMatch(missingCache, /Out: 4 \(partial\)/);

const complete = render(record, {
    requests: 1,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
});
assert.match(complete, /In: 100/);
assert.match(complete, /Cached: 0/);
assert.match(complete, /Out: 4/);
assert.doesNotMatch(complete, /Out: 4 \(partial\)/);

const measuredZero = render({ tokens_in: 0, tokens_cached: 0, tokens_out: 0 }, {
    requests: 1,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
});
assert.match(measuredZero, /In: 0/);
assert.match(measuredZero, /Cached: 0/);
assert.match(measuredZero, /Out: 0/);

const interruptedCheckpoint = render({
    tokens_in: 100,
    tokens_cached: 20,
    tokens_out: 4,
    checkpoint_session: {
        provider_calls_recorded: 1,
        provider_calls_attempted: null,
        usage_complete: false,
    },
});
assert.match(interruptedCheckpoint, /In: Unknown/);
assert.match(interruptedCheckpoint, /Cached: 20 \(partial\)/);
assert.match(interruptedCheckpoint, /Out: 4 \(partial\)/);
assert.match(interruptedCheckpoint, /Usage incomplete/);

const completedCheckpoint = render({
    tokens_in: 100,
    tokens_cached: 20,
    tokens_out: 4,
    checkpoint_session: {
        provider_calls_recorded: 1,
        provider_calls_attempted: 1,
        usage_complete: true,
    },
});
assert.match(completedCheckpoint, /In: 80/);
assert.match(completedCheckpoint, /Cached: 20/);
assert.match(completedCheckpoint, /Out: 4/);
assert.doesNotMatch(completedCheckpoint, /partial|incomplete/i);

const unconfirmedZeroCheckpoint = render({
    tokens_in: 0,
    tokens_cached: 0,
    tokens_out: 0,
    checkpoint_session: { provider_calls_recorded: 1, usage_complete: false },
});
assert.match(unconfirmedZeroCheckpoint, /Unavailable/);
assert.match(unconfirmedZeroCheckpoint, /Usage incomplete/);

const measuredZeroCheckpoint = render({
    tokens_in: 0,
    tokens_cached: 0,
    tokens_out: 0,
    checkpoint_session: { provider_calls_recorded: 1, usage_complete: true },
});
assert.match(measuredZeroCheckpoint, /In: 0/);
assert.match(measuredZeroCheckpoint, /Cached: 0/);
assert.match(measuredZeroCheckpoint, /Out: 0/);

const zeroCallCheckpoint = render({
    tokens_in: 0,
    tokens_cached: null,
    tokens_out: 0,
    checkpoint_session: {
        provider_calls_recorded: 0,
        provider_calls_attempted: 0,
        usage_complete: true,
    },
});
assert.equal(zeroCallCheckpoint, 'No provider requests recorded');

const unconfirmedCallCheckpoint = render({
    tokens_in: 0,
    tokens_cached: 0,
    tokens_out: 0,
    checkpoint_session: { provider_calls_recorded: 0, usage_complete: false },
}, {
    requests: 0,
    responses_with_usage: 0,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
});
assert.match(unconfirmedCallCheckpoint, /Unavailable/);
assert.match(unconfirmedCallCheckpoint, /checkpoint usage incomplete/);
assert.doesNotMatch(unconfirmedCallCheckpoint, /No provider requests recorded/);

const contradictoryCheckpoint = render({
    tokens_in: 0,
    tokens_cached: 0,
    tokens_out: 0,
    checkpoint_session: {
        provider_calls_recorded: 0,
        provider_calls_attempted: 0,
        usage_complete: true,
    },
}, {
    requests: 1,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
});
assert.doesNotMatch(contradictoryCheckpoint, /No provider requests recorded/);

const completedCheckpointWithoutCache = render({
    tokens_in: 100,
    tokens_cached: null,
    tokens_out: 4,
    checkpoint_session: { provider_calls_recorded: 1, usage_complete: true },
});
assert.match(completedCheckpointWithoutCache, /In: Unknown/);
assert.match(completedCheckpointWithoutCache, /Cached: Unknown/);
assert.match(completedCheckpointWithoutCache, /Out: 4/);
assert.doesNotMatch(completedCheckpointWithoutCache, /partial/);

console.log('Review token rendering checks passed');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const html = fs.readFileSync(path.join(__dirname, '../static/index.html'), 'utf8');
const start = html.indexOf('function renderReviewTokens(');
const end = html.indexOf('function renderActiveHarnessUsage(', start);
assert(start >= 0 && end > start, 'review token renderer not found');

const render = vm.runInNewContext(
    `(function() { ${html.slice(start, end)}; return renderReviewTokens; })()`,
    { formatTokenCount: value => String(value) },
);

const record = { tokens_in: 100, tokens_cached: 0, tokens_out: 4 };
const coverage = {
    requests: 2,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
};

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
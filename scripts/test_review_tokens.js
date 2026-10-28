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
    assert.match(result, /incomplete/);
}

const partial = render({ ...record, tokens_cached: 20 }, {
    ...coverage,
    responses_without_usage: 1,
});
assert.match(partial, /In: Unknown/);
assert.match(partial, /Cached: 20 \(partial\)/);

const missingCache = render(record, {
    requests: 1,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 1,
});
assert.match(missingCache, /In: Unknown/);
assert.match(missingCache, /Cached: Unknown/);

const complete = render(record, {
    requests: 1,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
});
assert.match(complete, /In: 100/);
assert.match(complete, /Cached: 0/);

const measuredZero = render({ tokens_in: 0, tokens_cached: 0, tokens_out: 0 }, {
    requests: 1,
    responses_with_usage: 1,
    responses_without_usage: 0,
    responses_without_cached_usage: 0,
});
assert.match(measuredZero, /In: 0/);
assert.match(measuredZero, /Cached: 0/);

console.log('Review token rendering checks passed');
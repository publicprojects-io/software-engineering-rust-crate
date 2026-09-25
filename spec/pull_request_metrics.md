# `pull_request_metrics` — Pull Request and Code Review Metrics

Rust module: [`src/pull_request_metrics.rs`](../src/pull_request_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/02-09-pull-request-and-code-review-metrics.md`

## Formula

```text
Time to first review        = t(first substantive comment or approval) - t(opened)
Reviewer load concentration = max(reviews per reviewer) / mean(reviews per reviewer)
```

## Public API

### `time_to_first_review`

```rust
pub fn time_to_first_review(opened_at: f64, first_response_at: f64) -> f64
```

Time to first review: the interval from a pull request being opened to a

### `reviewer_load_concentration_ratio`

```rust
pub fn reviewer_load_concentration_ratio(reviews_per_reviewer: &[f64]) -> Option<f64>
```

Reviewer load concentration ratio: the busiest reviewer's review count

## Sources

- Chapter 2.9, Pull request and code review metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/02-09-pull-request-and-code-review-metrics.md

# `satisfaction_metrics` — Satisfaction and Well-Being Metrics

Rust module: [`src/satisfaction_metrics.rs`](../src/satisfaction_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/03-02-satisfaction-and-well-being-metrics.md`

## Formula

```text
Satisfaction net score = ((promoters - detractors) / total_respondents) × 100
    (an employee Net Promoter-style score, ranging roughly -100 to +100)

Declining  when current_score < previous_score - decline_threshold
```

## Public API

### `satisfaction_net_score`

```rust
pub fn satisfaction_net_score(promoters: f64, detractors: f64, total_respondents: f64) -> Option<f64>
```

Employee-satisfaction Net Promoter-style score: the percentage of

### `is_satisfaction_declining`

```rust
pub fn is_satisfaction_declining(previous_score: f64, current_score: f64, decline_threshold: f64) -> bool
```

Whether a satisfaction score has declined enough between two

## Sources

- Chapter 3.2, Satisfaction and well-being metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/03-02-satisfaction-and-well-being-metrics.md

# `feature_adoption` — Feature Adoption and Usage Metrics

Rust module: [`src/feature_adoption.rs`](../src/feature_adoption.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-02-feature-adoption-and-usage-metrics.md`

## Formula

```text
Initial adoption %  = tried_at_least_once / target_audience × 100
Retained adoption % = still_using_after_n_weeks / initially_tried × 100

tried_at_least_once     = people in the target audience who tried the
                           feature at least once
target_audience         = the specific population the feature was built
                           for (not the whole user base)
still_using_after_n_weeks = of those initial triers, how many are still
                           using the feature after a meaningful period
                           (e.g. four or eight weeks)
initially_tried          = the initial-trial count (the denominator for
                           retention, distinct from target_audience)
```

## Public API

### `initial_adoption_percent`

```rust
pub fn initial_adoption_percent(
    tried_at_least_once: f64,
    target_audience: f64,
) -> Option<f64>
```

Initial adoption percentage: target audience who tried a feature at

### `retained_adoption_percent`

```rust
pub fn retained_adoption_percent(
    still_using_after_n_weeks: f64,
    initially_tried: f64,
) -> Option<f64>
```

Retained adoption percentage: initial triers still using the feature

## Sources

- Chapter 5.2, Feature adoption and usage metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/05-02-feature-adoption-and-usage-metrics.md

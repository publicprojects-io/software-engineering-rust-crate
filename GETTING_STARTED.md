# Getting started

## Install

```toml
[dependencies]
software-engineering = "1.1"
```

## Calculate one metric

Functions take already-collected values; the crate does not fetch data:

```rust
use software_engineering::dora_metrics::change_failure_rate_percent;

let rate = change_failure_rate_percent(3.0, 40.0);
assert_eq!(rate, Some(7.5));
```

## Handle an undefined ratio

Zero denominators return `None`:

```rust
use software_engineering::cycle_time::stage_percent_of_cycle;

let Some(review_share) = stage_percent_of_cycle(3.0, 10.0) else {
    // No complete cycle was observed, so there is no meaningful percentage.
    return;
};
assert!((review_share - 30.0).abs() < 1e-9);
```

In a function returning `Option`, propagation is often clearer:

```rust
use software_engineering::return_on_investment::roi;

fn percent_roi(benefit: f64, cost: f64) -> Option<f64> {
    Some(roi(benefit, cost)? * 100.0)
}
assert_eq!(percent_roi(120.0, 100.0), Some(20.0));
```

## Combine independent metrics

Use several signals to prevent a local improvement from hiding a regression.
For example, pair cycle-time change with defect-rate change using
`ai_assisted_development::is_genuine_productivity_gain`. See the runnable
examples for cross-module scenarios.

## Validate inputs

The crate intentionally does not impose organization-specific constraints.
Check that counts and durations are non-negative and finite at your ingestion
boundary. A zero denominator is the one universally undefined case handled by
the formula APIs themselves.

## Explore

Run `cargo doc --no-deps --open` for formulas, pitfalls, sources, and examples.
The module index in `README.md` groups APIs by the decisions they support.

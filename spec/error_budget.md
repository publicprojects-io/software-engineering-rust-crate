# `error_budget` — SLIs, SLOs, and Error Budgets

Rust module: [`src/error_budget.rs`](../src/error_budget.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/06-01-slis-slos-and-error-budgets.md`

## Formula

```text
Error budget (minutes) = (100 − SLO%) / 100 × period days × 24 × 60
Burn rate               = actual downtime / error budget
Budget exhausted         when burn rate ≥ 1.0
```

## Public API

### `error_budget_minutes`

```rust
pub fn error_budget_minutes(slo_percent: f64, period_days: f64) -> f64
```

The allowed downtime, in minutes, implied by an availability SLO over a

### `error_budget_burn_rate`

```rust
pub fn error_budget_burn_rate(actual_downtime_minutes: f64, budget_minutes: f64) -> Option<f64>
```

How much of the error budget has been spent.

### `is_error_budget_exhausted`

```rust
pub fn is_error_budget_exhausted(actual_downtime_minutes: f64, budget_minutes: f64) -> Option<bool>
```

Whether the error budget is exhausted: burn rate at or above `1.0`.

## Sources

- Chapter 6.1, SLIs, SLOs, and error budgets.

Topic doc: software-engineering-metrics/locales/en-001/chapters/06-01-slis-slos-and-error-budgets.md

# `on_call_metrics` — On-Call, Capacity, and Operational Load Metrics

Rust module: [`src/on_call_metrics.rs`](../src/on_call_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/06-03-on-call-capacity-and-operational-load-metrics.md`

## Formula

```text
Paging concentration (%)      = busiest engineer's pages / total pages × 100
On-call frequency ratio        = weeks on call / total weeks
Exceeds sustainable frequency   when on-call frequency ratio > max ratio
                                  (commonly 0.25, "no more than one week in four")
```

## Public API

### `paging_concentration_percent`

```rust
pub fn paging_concentration_percent(busiest_engineer_pages: f64, total_pages: f64) -> Option<f64>
```

Paging concentration: how much of the team's total paging load fell on

### `on_call_frequency_ratio`

```rust
pub fn on_call_frequency_ratio(weeks_on_call: f64, total_weeks: f64) -> Option<f64>
```

On-call frequency ratio: the fraction of weeks an engineer spent on

### `exceeds_sustainable_on_call_frequency`

```rust
pub fn exceeds_sustainable_on_call_frequency(
    weeks_on_call: f64,
    total_weeks: f64,
    max_ratio: f64,
) -> Option<bool>
```

Whether an engineer's on-call frequency exceeds a given sustainable

## Sources

- Chapter 6.3, On-call, capacity, and operational load metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/06-03-on-call-capacity-and-operational-load-metrics.md

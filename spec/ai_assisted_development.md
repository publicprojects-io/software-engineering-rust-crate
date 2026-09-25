# `ai_assisted_development` — Measuring AI-Assisted Software Development

Rust module: [`src/ai_assisted_development.rs`](../src/ai_assisted_development.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/07-02-measuring-ai-assisted-software-development.md`

## Formula

```text
Net cycle-time change (%) = (cycle time before − cycle time after) / cycle time before × 100
Genuine gain                 when cycle time improved AND defect rate did not worsen
```

## Public API

### `net_cycle_time_change_percent`

```rust
pub fn net_cycle_time_change_percent(cycle_time_before_hours: f64, cycle_time_after_hours: f64) -> Option<f64>
```

Net cycle-time change: the percentage change in full cycle time

### `is_genuine_productivity_gain`

```rust
pub fn is_genuine_productivity_gain(
    cycle_time_before_hours: f64,
    cycle_time_after_hours: f64,
    defect_rate_before_percent: f64,
    defect_rate_after_percent: f64,
) -> bool
```

Whether an observed cycle-time speedup is a genuine productivity gain

## Sources

- Chapter 7.2, Measuring AI-assisted software development.

Topic doc: software-engineering-metrics/locales/en-001/chapters/07-02-measuring-ai-assisted-software-development.md

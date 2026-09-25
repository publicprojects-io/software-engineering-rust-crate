# `cycle_time` — Cycle Time and Its Components

Rust module: [`src/cycle_time.rs`](../src/cycle_time.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/02-06-cycle-time-and-its-components.md`

## Formula

```text
Cycle time = coding + pickup + review + test + deploy

coding  = first commit → pull request opened
pickup  = pull request opened → first review
review  = first review → approval
test    = time spent in automated/manual verification
deploy  = approval → production
```

## Public API

### `cycle_time`

```rust
pub fn cycle_time(coding: f64, pickup: f64, review: f64, test: f64, deploy: f64) -> f64
```

Cycle time: the sum of a change's five named engineering stages.

### `stage_percent_of_cycle`

```rust
pub fn stage_percent_of_cycle(stage_duration: f64, total_cycle_time: f64) -> Option<f64>
```

A single stage's share of total cycle time, as a percentage.

## Sources

- Chapter 2.6, "Cycle time and its components."

Topic doc: software-engineering-metrics/locales/en-001/chapters/02-06-cycle-time-and-its-components.md

# `lean_value_stream_metrics` — Lean Value Stream Metrics

Rust module: [`src/lean_value_stream_metrics.rs`](../src/lean_value_stream_metrics.rs)

Source chapter: `02-08-lean-value-stream-metrics.md`

## Formula

```text
%C/A = usable units without rework / total units × 100%
Rolled throughput yield = %C/A(stage 1) × %C/A(stage 2) × ... × %C/A(stage N)
Takt time = available working time / customer demand over that period
```

## Public API

### `percent_complete_and_accurate`

```rust
pub fn percent_complete_and_accurate(usable_without_rework: f64, total_units: f64) -> Option<f64>
```

Percent complete and accurate (%C/A): the share of a stage's output that

### `rolled_throughput_yield`

```rust
pub fn rolled_throughput_yield(stage_pca_fractions: &[f64]) -> f64
```

Rolled throughput yield: the product of every stage's %C/A fraction

### `takt_time`

```rust
pub fn takt_time(available_working_time: f64, customer_demand: f64) -> Option<f64>
```

Takt time: the maximum acceptable time to complete a unit to cleanly

## Sources

- Rother, Mike, and John Shook. *Learning to See: Value Stream Mapping to
  Create Value and Eliminate Muda*. Lean Enterprise Institute, 1999.
- Ohno, Taiichi. *Toyota Production System: Beyond Large-Scale
  Production*. Productivity Press, 1988.

Topic doc: 02-08-lean-value-stream-metrics.md

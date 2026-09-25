# `queueing_theory` — Queueing Theory

Rust module: [`src/queueing_theory.rs`](../src/queueing_theory.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/02-07-queueing-theory.md`

## Formula

```text
Utilization       = arrival rate / service rate
Queue is stable    when utilization < 1.0
```

## Public API

### `utilization`

```rust
pub fn utilization(arrival_rate: f64, service_rate: f64) -> Option<f64>
```

Utilization: arrival rate divided by service rate for a shared,

### `is_queue_stable`

```rust
pub fn is_queue_stable(utilization: f64) -> bool
```

Whether a queue is stable: utilization strictly less than 1.0.

## Sources

- Chapter 2.7, "Queueing theory."
- Little, John D. C. "A Proof for the Queuing Formula: L = λW." *Operations
  Research*, 1961.

Topic doc: software-engineering-metrics/locales/en-001/chapters/02-07-queueing-theory.md

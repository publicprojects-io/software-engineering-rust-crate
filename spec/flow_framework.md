# `flow_framework` — Flow Framework

Rust module: [`src/flow_framework.rs`](../src/flow_framework.rs)

Source chapter: `02-01-the-flow-framework.md,`

## Formula

```text
Flow time  = t(delivery) − t(entry)
Flow load  = count of items currently active or waiting in the value stream
Little's law: flow load (WIP) = arrival rate × flow time (cycle time)
Flow efficiency = active time / total elapsed time × 100%

t(entry)     = when a flow item enters the value stream
t(delivery)  = when a flow item is delivered
arrival rate = new items entering the value stream per unit time
```

## Public API

### `flow_velocity`

```rust
pub fn flow_velocity(items_completed: f64, period: f64) -> Option<f64>
```

Flow velocity: count of flow items completed per unit time.

### `flow_distribution_percent`

```rust
pub fn flow_distribution_percent(items_of_type: f64, total_completed_items: f64) -> Option<f64>
```

Flow distribution: the percentage share of one flow item type among all

### `flow_time`

```rust
pub fn flow_time(entry: f64, delivery: f64) -> f64
```

Flow time: total elapsed time from a flow item entering the value stream

### `flow_load_from_items`

```rust
pub fn flow_load_from_items(active: u32, waiting: u32) -> u32
```

Flow load: total count of flow items currently active or waiting in the

### `littles_law_wip`

```rust
pub fn littles_law_wip(arrival_rate: f64, flow_time: f64) -> f64
```

Little's law, solved for flow load (work in process): `WIP = arrival rate

### `littles_law_flow_time`

```rust
pub fn littles_law_flow_time(wip: f64, arrival_rate: f64) -> Option<f64>
```

Little's law, solved for flow time: `flow time = WIP / arrival rate`.

### `flow_efficiency_percent`

```rust
pub fn flow_efficiency_percent(active_time: f64, total_elapsed_time: f64) -> Option<f64>
```

Flow efficiency: the percentage of total elapsed time that was active

## Sources

- Kersten, Mik. *Project to Product: How to Survive and Thrive in the Age
  of Digital Disruption with the Flow Framework*. IT Revolution Press,
  2018.
- Little, John D. C. "A Proof for the Queuing Formula: L = λW." *Operations
  Research*, 1961.

Topic doc: 02-01-the-flow-framework.md, 02-03-flow-velocity-and-flow-distribution.md,
02-04-flow-time-and-flow-load.md, 02-05-flow-efficiency-and-work-in-process.md

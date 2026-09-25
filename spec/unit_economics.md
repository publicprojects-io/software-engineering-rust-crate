# `unit_economics` — Cost and Unit Economics of Engineering

Rust module: [`src/unit_economics.rs`](../src/unit_economics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-04-cost-and-unit-economics-of-engineering.md`

## Formula

```text
Total engineering cost = people cost + infrastructure cost + tooling cost
Unit cost                = total cost / units delivered
```

## Public API

### `total_engineering_cost`

```rust
pub fn total_engineering_cost(people_cost: f64, infrastructure_cost: f64, tooling_cost: f64) -> f64
```

The sum of engineering's three distinct cost components.

### `unit_cost`

```rust
pub fn unit_cost(total_cost: f64, units_delivered: f64) -> Option<f64>
```

Cost per genuine unit of value delivered, such as cost per customer

## Sources

- Chapter 5.4, Cost and unit economics of engineering.
- `FinOps` Foundation, *`FinOps` Framework*.

Topic doc: software-engineering-metrics/locales/en-001/chapters/05-04-cost-and-unit-economics-of-engineering.md

# `unit_economics` — Cost and Unit Economics of Engineering

Rust module: [`src/unit_economics.rs`](../src/unit_economics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-04-cost-and-unit-economics-of-engineering.md`

## Formula

```text
Total engineering cost = people cost + infrastructure cost + tooling cost
Unit cost                = total cost / units delivered
```

## Money

[`total_engineering_cost`] and [`unit_cost`] take plain `f64` amounts.
For currency-checked accounting, use [`rusty_money::Money`] directly
rather than through a wrapper this crate provides — its own `add`/`div`
already return `Result`, rejecting cost components quoted in different
currencies instead of silently treating them as the same unit:

```rust
use rusty_money::{Money, iso};
use software_engineering::unit_economics::unit_cost;

let people = Money::from_major(500_000, iso::USD);
let infrastructure = Money::from_major(120_000, iso::USD);
let tooling = Money::from_major(30_000, iso::USD);
let total = people.add(infrastructure).unwrap().add(tooling).unwrap();
assert_eq!(total, Money::from_major(650_000, iso::USD));

let cost_per_customer = unit_cost(total.to_f64_lossy(), 10_000.0).unwrap();
assert!((cost_per_customer - 65.0).abs() < 1e-9);

// Dividing by zero units is rejected rather than producing infinity.
assert!(total.div(0).is_err());
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

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
[`total_engineering_cost_money`] and [`unit_cost_money`] do the
equivalent calculation over [`rusty_money::Money`], so summing cost
components quoted in different currencies is rejected as an error
instead of silently treated as the same unit.

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

### `total_engineering_cost_money`

```rust
pub fn total_engineering_cost_money<'a, T: rusty_money::FormattableCurrency>(
    people_cost: rusty_money::Money<'a, T>,
    infrastructure_cost: rusty_money::Money<'a, T>,
    tooling_cost: rusty_money::Money<'a, T>,
) -> Result<rusty_money::Money<'a, T>, rusty_money::MoneyError>
```

The sum of engineering's three distinct cost components, computed over

### `unit_cost_money`

```rust
pub fn unit_cost_money<T: rusty_money::FormattableCurrency>(
    total_cost: rusty_money::Money<'_, T>,
    units_delivered: u32,
) -> Result<rusty_money::Money<'_, T>, rusty_money::MoneyError>
```

Cost per genuine unit of value delivered, computed over

## Sources

- Chapter 5.4, Cost and unit economics of engineering.
- `FinOps` Foundation, *`FinOps` Framework*.

Topic doc: software-engineering-metrics/locales/en-001/chapters/05-04-cost-and-unit-economics-of-engineering.md

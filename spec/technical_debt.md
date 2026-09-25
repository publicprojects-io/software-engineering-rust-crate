# `technical_debt` — Technical Debt Measurement

Rust module: [`src/technical_debt.rs`](../src/technical_debt.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/04-05-technical-debt-measurement.md`

## Formula

```text
Debt carrying cost = (velocity tax + elevated defect cost) × periods

velocity tax          = extra cost per period from related work going slower
elevated defect cost  = extra expected defect cost per period from carrying the item
periods                = number of periods the item is left unfixed
```

## Public API

### `debt_carrying_cost`

```rust
pub fn debt_carrying_cost(
    velocity_tax_per_period: f64,
    elevated_defect_cost_per_period: f64,
    periods: f64,
) -> f64
```

Debt carrying cost: the ongoing cost of leaving a debt item unfixed.

## Sources

- Chapter 4.5, Technical debt measurement.
- Cunningham, Ward, "The `WyCash` Portfolio Management System," *OOPSLA*
  (1992).
- Kruchten, Philippe, Robert Nord, and Ipek Ozkaya, *Managing Technical
  Debt: Reducing Friction in Software Development*.

Topic doc: software-engineering-metrics/locales/en-001/chapters/04-05-technical-debt-measurement.md

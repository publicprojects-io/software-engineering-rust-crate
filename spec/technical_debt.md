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

## Money

[`debt_carrying_cost`] takes plain `f64` amounts. For currency-checked
accounting, use [`rusty_money::Money`] directly rather than through a
wrapper this crate provides — its own `add`/`mul` already return
`Result`, rejecting a currency mismatch (a USD velocity tax against a
EUR defect cost, say) instead of silently summing incompatible amounts,
so this formula needs no adapter to use it that way:

```rust
use rusty_money::{Money, iso};

// $2,000/month velocity tax + $500/month elevated defect cost,
// carried for 12 months = $30,000.
let velocity_tax = Money::from_major(2_000, iso::USD);
let defect_cost = Money::from_major(500, iso::USD);
let cost = velocity_tax.add(defect_cost).unwrap().mul(12).unwrap();
assert_eq!(cost, Money::from_major(30_000, iso::USD));

// Mismatched currencies are rejected rather than silently summed.
let eur_defect_cost = Money::from_major(500, iso::EUR);
assert!(velocity_tax.add(eur_defect_cost).is_err());
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

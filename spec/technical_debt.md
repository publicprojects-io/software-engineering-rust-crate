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

[`debt_carrying_cost`] takes plain `f64` amounts and leaves currency
bookkeeping to the caller. [`debt_carrying_cost_money`] does the same
calculation over [`rusty_money::Money`] instead, so a currency mismatch
between the two per-period costs (mixing USD and EUR, say) is caught as
an error rather than silently summed as if they were the same unit.

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

### `debt_carrying_cost_money`

```rust
pub fn debt_carrying_cost_money<'a, T: rusty_money::FormattableCurrency>(
    velocity_tax_per_period: rusty_money::Money<'a, T>,
    elevated_defect_cost_per_period: rusty_money::Money<'a, T>,
    periods: u32,
) -> Result<rusty_money::Money<'a, T>, rusty_money::MoneyError>
```

Debt carrying cost, computed over [`rusty_money::Money`] instead of

## Sources

- Chapter 4.5, Technical debt measurement.
- Cunningham, Ward, "The `WyCash` Portfolio Management System," *OOPSLA*
  (1992).
- Kruchten, Philippe, Robert Nord, and Ipek Ozkaya, *Managing Technical
  Debt: Reducing Friction in Software Development*.

Topic doc: software-engineering-metrics/locales/en-001/chapters/04-05-technical-debt-measurement.md

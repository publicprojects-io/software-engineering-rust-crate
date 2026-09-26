# `return_on_investment` — Return on Investment for Engineering Initiatives

Rust module: [`src/return_on_investment.rs`](../src/return_on_investment.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-05-return-on-investment-for-engineering-initiatives.md`

## Formula

```text
ROI          = (benefit − cost) / cost
ROI range    = (roi(conservative benefit, cost), roi(optimistic benefit, cost))
```

## Money

[`roi`] and [`roi_range`] take plain `f64` amounts. For currency-checked
accounting, use [`rusty_money::Money`] directly rather than through a
wrapper this crate provides — its own `sub` already returns `Result`,
rejecting a benefit and cost quoted in different currencies (USD
against EUR, say) instead of silently treating them as the same unit,
and [`rusty_money::Money::to_f64_lossy`] converts the net benefit and
cost into the same plain proportion [`roi`] returns:

```rust
use rusty_money::{Money, iso};
use software_engineering::return_on_investment::roi;

let benefit = Money::from_major(300_000, iso::USD);
let cost = Money::from_major(100_000, iso::USD);

// rusty_money's own sub() catches a currency mismatch before it ever
// reaches roi(), which only ever sees plain, same-unit f64 amounts.
let net_benefit = benefit.sub(cost).unwrap();
assert_eq!(net_benefit, Money::from_major(200_000, iso::USD));

let r = roi(benefit.to_f64_lossy(), cost.to_f64_lossy()).unwrap();
assert!((r - 2.0).abs() < 1e-9);

// Mismatched currencies are rejected rather than silently subtracted.
let eur_cost = Money::from_major(100_000, iso::EUR);
assert!(benefit.sub(eur_cost).is_err());
```

## Public API

### `roi`

```rust
pub fn roi(benefit: f64, cost: f64) -> Option<f64>
```

Return on investment: net benefit as a proportion of cost.

### `roi_range`

```rust
pub fn roi_range(conservative_benefit: f64, optimistic_benefit: f64, cost: f64) -> Option<(f64, f64)>
```

ROI expressed as a conservative-to-optimistic range against the same

## Sources

- Chapter 5.5, Return on investment for engineering initiatives.

Topic doc: software-engineering-metrics/locales/en-001/chapters/05-05-return-on-investment-for-engineering-initiatives.md

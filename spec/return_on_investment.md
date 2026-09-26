# `return_on_investment` — Return on Investment for Engineering Initiatives

Rust module: [`src/return_on_investment.rs`](../src/return_on_investment.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-05-return-on-investment-for-engineering-initiatives.md`

## Formula

```text
ROI          = (benefit − cost) / cost
ROI range    = (roi(conservative benefit, cost), roi(optimistic benefit, cost))
```

## Money

[`roi`] and [`roi_range`] take plain `f64` amounts. [`roi_money`] and
[`net_benefit_money`] do the equivalent calculation over
[`rusty_money::Money`], so a benefit and cost quoted in different
currencies (USD benefit against a EUR cost, say) are rejected as an
error instead of silently treated as the same unit.

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

### `net_benefit_money`

```rust
pub fn net_benefit_money<'a, T: rusty_money::FormattableCurrency>(
    benefit: rusty_money::Money<'a, T>,
    cost: rusty_money::Money<'a, T>,
) -> Result<rusty_money::Money<'a, T>, rusty_money::MoneyError>
```

Net benefit (`benefit − cost`) computed over [`rusty_money::Money`]

### `roi_money`

```rust
pub fn roi_money<'a, T: rusty_money::FormattableCurrency>(
    benefit: rusty_money::Money<'a, T>,
    cost: rusty_money::Money<'a, T>,
) -> Result<f64, rusty_money::MoneyError>
```

Return on investment computed over [`rusty_money::Money`] instead of

## Sources

- Chapter 5.5, Return on investment for engineering initiatives.

Topic doc: software-engineering-metrics/locales/en-001/chapters/05-05-return-on-investment-for-engineering-initiatives.md

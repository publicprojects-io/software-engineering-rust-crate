# `return_on_investment` — Return on Investment for Engineering Initiatives

Rust module: [`src/return_on_investment.rs`](../src/return_on_investment.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-05-return-on-investment-for-engineering-initiatives.md`

## Formula

```text
ROI          = (benefit − cost) / cost
ROI range    = (roi(conservative benefit, cost), roi(optimistic benefit, cost))
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

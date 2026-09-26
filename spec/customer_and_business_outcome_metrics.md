# `customer_and_business_outcome_metrics` — Customer and Business Outcome Metrics

Rust module: [`src/customer_and_business_outcome_metrics.rs`](../src/customer_and_business_outcome_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-03-customer-and-business-outcome-metrics.md`

## Formula

```text
Net revenue retention % =
    (revenue_at_period_end_from_starting_cohort / starting_revenue) × 100
    (deliberately excludes new-customer revenue)

Honestly scoped  when contributing_factors_considered >= 2
```

## Public API

### `net_revenue_retention_percent`

```rust
pub fn net_revenue_retention_percent(
    starting_revenue: f64,
    revenue_at_period_end_from_starting_cohort: f64,
) -> Option<f64>
```

Net revenue retention: the percentage of a starting cohort's revenue

### `is_outcome_claim_honestly_scoped`

```rust
pub fn is_outcome_claim_honestly_scoped(contributing_factors_considered: u32) -> bool
```

Whether an engineering-attributed outcome claim is honestly scoped,

## Sources

- Chapter 5.3, Customer and business outcome metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/05-03-customer-and-business-outcome-metrics.md

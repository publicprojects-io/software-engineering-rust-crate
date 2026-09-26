//! # Customer and Business Outcome Metrics
//!
//! This module widens the lens beyond feature-level adoption to the full
//! range of customer and business outcomes an organization actually
//! cares about: revenue retained or grown, customer satisfaction and
//! loyalty, cost reduction, and, for public-sector organizations, the
//! citizen outcomes a mission exists to serve. These outcomes are rarely
//! attributable to engineering alone, and pretending otherwise produces
//! a false-precision problem. The productive response is not to give up
//! on connecting engineering work to business outcomes, but to be honest
//! about the connection's strength and to use converging evidence rather
//! than false-precision claims of direct causation.
//!
//! ## Formula
//!
//! ```text
//! Net revenue retention % =
//!     (revenue_at_period_end_from_starting_cohort / starting_revenue) × 100
//!     (deliberately excludes new-customer revenue)
//!
//! Honestly scoped  when contributing_factors_considered >= 2
//! ```
//!
//! ## Why it matters
//!
//! Outcomes at the business level are rarely attributable to engineering
//! alone: they also depend on sales, marketing, market conditions, and
//! product strategy decisions made well outside engineering's control. A
//! well-run engineering organization can show that its work correlates
//! with, contributes to, and sometimes directly drives specific business
//! outcomes, without claiming sole credit for outcomes that also depend
//! on other functions. Net revenue retention is a useful outcome metric
//! precisely because it isolates the value of customers already won —
//! independent of new-customer acquisition — but it still says nothing
//! about *why* it moved without converging evidence from more than one
//! contributing factor.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::customer_and_business_outcome_metrics::{
//!     net_revenue_retention_percent, is_outcome_claim_honestly_scoped,
//! };
//!
//! // A cohort that started the period at $200,000 in revenue and, after
//! // expansions outweighed churn, ended at $230,000: 115% net revenue
//! // retention.
//! let nrr = net_revenue_retention_percent(200_000.0, 230_000.0).unwrap();
//! assert!((nrr - 115.0).abs() < 1e-9);
//!
//! // Claiming that improvement was caused by engineering alone, with no
//! // other contributing factor considered, is the false-precision claim
//! // this chapter warns against.
//! assert!(!is_outcome_claim_honestly_scoped(1));
//! assert!(is_outcome_claim_honestly_scoped(2));
//! ```
//!
//! ## Pitfalls
//!
//! - **Claiming direct causation without checking for confounds** —
//!   overstates certainty and risks credibility damage if challenged.
//! - **Presenting engineering and outcome metrics side by side with no
//!   documented causal chain** — invites the audience to infer a
//!   connection that may not actually hold.
//! - **Treating quantitative outcome data as inherently more
//!   authoritative than qualitative evidence** — misses context and
//!   explanatory power the numbers alone cannot provide.
//! - **Avoiding outcome claims entirely to sidestep attribution
//!   difficulty** — leaves engineering's actual business value
//!   undemonstrated and underappreciated.
//!
//! ## Sources
//!
//! - Chapter 5.3, Customer and business outcome metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/05-03-customer-and-business-outcome-metrics.md

/// Net revenue retention: the percentage of a starting cohort's revenue
/// retained after expansions, contractions, and churn — deliberately
/// excluding new-customer revenue, so it isolates how well the
/// organization keeps and grows the value of customers it already has.
///
/// `(revenue_at_period_end_from_starting_cohort / starting_revenue) × 100`.
/// A value above `100.0` means the existing cohort's expansions
/// outweighed its churn and contractions.
///
/// # Arguments
///
/// * `starting_revenue` — the starting cohort's revenue at the beginning
///   of the period.
/// * `revenue_at_period_end_from_starting_cohort` — how much of that same
///   cohort's revenue remains (after expansion, contraction, and churn)
///   at the end of the period, excluding any new customers.
///
/// # Returns
///
/// The net revenue retention percentage, or `None` if `starting_revenue`
/// is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::customer_and_business_outcome_metrics::net_revenue_retention_percent;
///
/// // Expansions outweighed churn: 115% retention.
/// let nrr = net_revenue_retention_percent(200_000.0, 230_000.0).unwrap();
/// assert!((nrr - 115.0).abs() < 1e-9);
/// // Churn and contraction outweighed expansion: 80% retention.
/// let nrr = net_revenue_retention_percent(200_000.0, 160_000.0).unwrap();
/// assert!((nrr - 80.0).abs() < 1e-9);
/// assert_eq!(net_revenue_retention_percent(0.0, 1.0), None);
/// ```
#[must_use]
pub fn net_revenue_retention_percent(
    starting_revenue: f64,
    revenue_at_period_end_from_starting_cohort: f64,
) -> Option<f64> {
    if starting_revenue == 0.0 {
        return None;
    }
    Some((revenue_at_period_end_from_starting_cohort / starting_revenue) * 100.0)
}

/// Whether an engineering-attributed outcome claim is honestly scoped,
/// per the chapter's explicit warning against false-precision causation
/// claims.
///
/// True iff `contributing_factors_considered >= 2` — converging evidence
/// from at least engineering plus one other factor (sales, marketing,
/// market conditions), rather than a single-factor claim of sole credit.
///
/// # Arguments
///
/// * `contributing_factors_considered` — how many distinct contributing
///   factors were considered when attributing the outcome.
///
/// # Returns
///
/// `true` if at least two contributing factors were considered.
///
/// # Examples
///
/// ```rust
/// use software_engineering::customer_and_business_outcome_metrics::is_outcome_claim_honestly_scoped;
///
/// // Claiming sole credit for engineering: not honestly scoped.
/// assert!(!is_outcome_claim_honestly_scoped(1));
/// // Converging evidence from engineering plus at least one other factor.
/// assert!(is_outcome_claim_honestly_scoped(2));
/// ```
#[must_use]
pub fn is_outcome_claim_honestly_scoped(contributing_factors_considered: u32) -> bool {
    contributing_factors_considered >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // "revenue retained or grown" — expansion among the existing cohort
    // outweighing churn yields retention above 100%.
    #[test]
    fn expansion_outweighing_churn_yields_retention_above_100_percent() {
        let nrr = net_revenue_retention_percent(200_000.0, 230_000.0).unwrap();
        assert!((nrr - 115.0).abs() < 1e-9);
    }

    #[test]
    fn churn_outweighing_expansion_yields_retention_below_100_percent() {
        let nrr = net_revenue_retention_percent(200_000.0, 160_000.0).unwrap();
        assert!((nrr - 80.0).abs() < 1e-9);
    }

    #[test]
    fn net_revenue_retention_is_none_for_zero_starting_revenue() {
        assert_eq!(net_revenue_retention_percent(0.0, 1.0), None);
    }

    // "outcomes at this level are rarely attributable to engineering
    // alone... use converging evidence rather than false-precision
    // claims of direct causation."
    #[test]
    fn a_single_contributing_factor_is_not_honestly_scoped() {
        assert!(!is_outcome_claim_honestly_scoped(1));
    }

    #[test]
    fn two_or_more_contributing_factors_are_honestly_scoped() {
        assert!(is_outcome_claim_honestly_scoped(2));
    }
}

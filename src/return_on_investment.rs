//! # Return on Investment for Engineering Initiatives
//!
//! **Return on investment (ROI)** turns a delivery or reliability
//! improvement into a financial case decision-makers outside engineering can
//! weigh directly against competing investments. Build the cost side from
//! full total cost of ownership, not just upfront development cost, and
//! build the benefit side from documented, honest outcome evidence rather
//! than an optimistic first-principles guess. Because both sides carry real
//! uncertainty, present ROI as a range — a conservative case and an
//! optimistic case — rather than a single, falsely precise number.
//!
//! ## Formula
//!
//! ```text
//! ROI          = (benefit − cost) / cost
//! ROI range    = (roi(conservative benefit, cost), roi(optimistic benefit, cost))
//! ```
//!
//! ## Why it matters
//!
//! A single point estimate that turns out to be wrong damages a case's
//! credibility far more than a well-explained range the actual outcome
//! falls within. An analysis process that is genuinely capable of
//! concluding "this is not worth it," and treats that as a legitimate
//! result rather than a failure of the analysis, is what keeps ROI
//! reporting trustworthy over time — an organization known for only ever
//! producing positive ROI cases quickly loses credibility, because
//! stakeholders correctly infer the analysis is not independent of the
//! decision it is meant to inform.
//!
//! ## Example
//!
//! A platform team's change-failure-rate improvement (see
//! [`crate::dora_metrics`]) avoids 17 failed deployments a year, each saving
//! $12,000 in incident cost, for a $204,000 annual benefit against a
//! $150,000 investment.
//!
//! ```rust
//! use software_engineering::return_on_investment::{roi, roi_range};
//!
//! let r = roi(300_000.0, 100_000.0).unwrap();
//! assert_eq!(r, 2.0);
//!
//! let benefit = 204_000.0;
//! let return_on_investment = roi(benefit, 150_000.0).unwrap();
//! assert!((return_on_investment - 0.36).abs() < 1e-9);
//!
//! // Present the same benefit as a conservative-to-optimistic range instead
//! // of one falsely precise number.
//! let (conservative, optimistic) = roi_range(150_000.0, 250_000.0, 150_000.0).unwrap();
//! assert_eq!(conservative, 0.0);
//! assert!((optimistic - (2.0 / 3.0)).abs() < 1e-9);
//! ```
//!
//! ## Money
//!
//! [`roi`] and [`roi_range`] take plain `f64` amounts. For currency-checked
//! accounting, use [`rusty_money::Money`] directly rather than through a
//! wrapper this crate provides — its own `sub` already returns `Result`,
//! rejecting a benefit and cost quoted in different currencies (USD
//! against EUR, say) instead of silently treating them as the same unit,
//! and [`rusty_money::Money::to_f64_lossy`] converts the net benefit and
//! cost into the same plain proportion [`roi`] returns:
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use software_engineering::return_on_investment::roi;
//!
//! let benefit = Money::from_major(300_000, iso::USD);
//! let cost = Money::from_major(100_000, iso::USD);
//!
//! // rusty_money's own sub() catches a currency mismatch before it ever
//! // reaches roi(), which only ever sees plain, same-unit f64 amounts.
//! let net_benefit = benefit.sub(cost).unwrap();
//! assert_eq!(net_benefit, Money::from_major(200_000, iso::USD));
//!
//! let r = roi(benefit.to_f64_lossy(), cost.to_f64_lossy()).unwrap();
//! assert!((r - 2.0).abs() < 1e-9);
//!
//! // Mismatched currencies are rejected rather than silently subtracted.
//! let eur_cost = Money::from_major(100_000, iso::EUR);
//! assert!(benefit.sub(eur_cost).is_err());
//! ```
//!
//! ## Pitfalls
//!
//! - **Costing only the upfront investment**, omitting ongoing maintenance,
//!   infrastructure, and opportunity cost — makes a case look cheaper than
//!   its full lifetime cost.
//! - **Inventing a benefit estimate from first principles** rather than
//!   grounding it in documented, measured, or comparable historical outcome
//!   data.
//! - **Presenting a single point estimate** instead of a range — a falsely
//!   precise number that damages credibility when it turns out wrong.
//! - **Never reporting a negative or marginal ROI finding** — a sign the
//!   analysis is not actually independent of the decision it informs.
//! - **Never closing the loop** — failing to compare actual outcomes against
//!   the projected range after the fact erodes the organization's future
//!   forecasting credibility.
//!
//! ## Sources
//!
//! - Chapter 5.5, Return on investment for engineering initiatives.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/05-05-return-on-investment-for-engineering-initiatives.md

/// Return on investment: net benefit as a proportion of cost.
///
/// `(benefit − cost) / cost`. A result of `2.0` means every $1 invested
/// returns $2 in net profit — a 3x total return.
///
/// # Arguments
///
/// * `benefit` — the total realized or projected benefit, in any currency
///   unit.
/// * `cost` — the total cost, including ongoing total cost of ownership, in
///   the same unit.
///
/// # Returns
///
/// The ROI as a proportion (not a percentage), or `None` if `cost` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::return_on_investment::roi;
///
/// assert_eq!(roi(300_000.0, 100_000.0), Some(2.0));
/// assert_eq!(roi(1.0, 0.0), None);
/// ```
#[must_use]
pub fn roi(benefit: f64, cost: f64) -> Option<f64> {
    if cost == 0.0 {
        return None;
    }
    Some((benefit - cost) / cost)
}

/// ROI expressed as a conservative-to-optimistic range against the same
/// cost, rather than a single, falsely precise number.
///
/// `(roi(conservative_benefit, cost), roi(optimistic_benefit, cost))`.
///
/// # Arguments
///
/// * `conservative_benefit` — the low, conservative-case benefit estimate.
/// * `optimistic_benefit` — the high, optimistic-case benefit estimate.
/// * `cost` — the total cost shared by both cases, in the same unit.
///
/// # Returns
///
/// A `(conservative_roi, optimistic_roi)` pair, or `None` if `cost` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::return_on_investment::roi_range;
///
/// let (conservative, optimistic) = roi_range(150_000.0, 250_000.0, 150_000.0).unwrap();
/// assert_eq!(conservative, 0.0);
/// assert!((optimistic - (2.0 / 3.0)).abs() < 1e-9);
/// assert_eq!(roi_range(1.0, 2.0, 0.0), None);
/// ```
#[must_use]
pub fn roi_range(conservative_benefit: f64, optimistic_benefit: f64, cost: f64) -> Option<(f64, f64)> {
    if cost == 0.0 {
        return None;
    }
    let conservative = (conservative_benefit - cost) / cost;
    let optimistic = (optimistic_benefit - cost) / cost;
    Some((conservative, optimistic))
}

#[cfg(test)]
mod tests {
    use super::*;

    // "ROI = (benefit − cost) / cost. every $1 invested returns $2 in net
    // profit, a 3x total return."
    #[test]
    fn roi_computes_net_benefit_over_cost() {
        let r = roi(300_000.0, 100_000.0).unwrap();
        assert!((r - 2.0).abs() < 1e-9);
    }

    #[test]
    fn roi_matches_the_change_failure_rate_worked_example() {
        let r = roi(204_000.0, 150_000.0).unwrap();
        assert!((r - 0.36).abs() < 1e-9);
    }

    #[test]
    fn roi_is_none_for_zero_cost() {
        assert_eq!(roi(1.0, 0.0), None);
    }

    // "Present ROI estimates as a range (a conservative case and an
    // optimistic case) rather than a single, falsely precise figure."
    #[test]
    fn roi_range_returns_conservative_and_optimistic_pair() {
        let (conservative, optimistic) = roi_range(150_000.0, 250_000.0, 150_000.0).unwrap();
        assert!((conservative - 0.0).abs() < 1e-9);
        assert!((optimistic - (2.0 / 3.0)).abs() < 1e-9);
    }

    #[test]
    fn roi_range_is_none_for_zero_cost() {
        assert_eq!(roi_range(1.0, 2.0, 0.0), None);
    }
}

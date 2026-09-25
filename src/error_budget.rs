//! # SLIs, SLOs, and Error Budgets
//!
//! A **service level indicator (SLI)** measures something users genuinely
//! experience, such as request success rate or latency. A **service level
//! objective (SLO)** sets a target for that indicator. The **error budget**
//! is the amount of unreliability the SLO permits — a spendable resource,
//! not something to hoard, that gives both engineering and operations a
//! shared, objective rule for when to ship faster and when to slow down for
//! reliability work.
//!
//! ## Formula
//!
//! ```text
//! Error budget (minutes) = (100 − SLO%) / 100 × period days × 24 × 60
//! Burn rate               = actual downtime / error budget
//! Budget exhausted         when burn rate ≥ 1.0
//! ```
//!
//! ## Why it matters
//!
//! Calculating the error budget directly from the SLO, and tracking spending
//! against it continuously, turns an abstract reliability target into an
//! operational rule: agree in advance, before any specific incident, what
//! happens when the budget is exhausted (a common, effective policy is that
//! feature work pauses and priority shifts to reliability work). This
//! predetermined rule removes the need to relitigate the trade-off under
//! pressure during every individual incident. A healthy, unspent budget is
//! not something to preserve untouched — it is permission to take
//! reasonable, deliberate risks.
//!
//! ## Example
//!
//! The chapter's own worked example: a 99.9% availability target over 30
//! days permits roughly 43 minutes of allowed downtime.
//!
//! ```rust
//! use software_engineering::error_budget::{
//!     error_budget_minutes, error_budget_burn_rate, is_error_budget_exhausted,
//! };
//!
//! let budget = error_budget_minutes(99.9, 30.0);
//! assert!((budget - 43.2).abs() < 1e-9);
//!
//! // 20 minutes of actual downtime against a ~43-minute budget: not exhausted.
//! let burn_rate = error_budget_burn_rate(20.0, budget).unwrap();
//! assert!(burn_rate < 1.0);
//! assert!(!is_error_budget_exhausted(20.0, budget).unwrap());
//! ```
//!
//! ## Pitfalls
//!
//! - **Setting an aspirational SLO with no evidence behind it** — produces a
//!   target the team cannot realistically track or act on.
//! - **Treating the error budget as something to preserve rather than
//!   spend** — a budget that never gets spent suggests an overly
//!   conservative team or an SLO set too loosely relative to actual
//!   achieved reliability.
//! - **No predetermined response to exhaustion** — forces the trade-off to
//!   be relitigated under pressure during every individual incident.
//! - **Reviewing SLOs only by inertia**, never against evidence of actual
//!   achieved reliability or changed user expectations.
//!
//! ## Sources
//!
//! - Chapter 6.1, SLIs, SLOs, and error budgets.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/06-01-slis-slos-and-error-budgets.md

/// The allowed downtime, in minutes, implied by an availability SLO over a
/// given period.
///
/// `(100.0 − slo_percent) / 100.0 × period_days × 24.0 × 60.0`.
///
/// # Arguments
///
/// * `slo_percent` — the availability target, e.g. `99.9` for 99.9%.
/// * `period_days` — the length of the budget period, in days.
///
/// # Returns
///
/// The allowed downtime for the period, in minutes.
///
/// # Examples
///
/// ```rust
/// use software_engineering::error_budget::error_budget_minutes;
///
/// // A 99.9% target over 30 days permits roughly 43 minutes of downtime.
/// let budget = error_budget_minutes(99.9, 30.0);
/// assert!((budget - 43.2).abs() < 1e-9);
/// ```
#[must_use]
pub fn error_budget_minutes(slo_percent: f64, period_days: f64) -> f64 {
    (100.0 - slo_percent) / 100.0 * period_days * 24.0 * 60.0
}

/// How much of the error budget has been spent.
///
/// `actual_downtime_minutes / budget_minutes`. A value at or above `1.0`
/// means the budget is exhausted or overspent.
///
/// # Arguments
///
/// * `actual_downtime_minutes` — actual downtime observed in the period, in
///   minutes.
/// * `budget_minutes` — the allowed downtime for the period, in minutes
///   (typically from [`error_budget_minutes`]).
///
/// # Returns
///
/// The burn rate as a proportion, or `None` if `budget_minutes` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::error_budget::error_budget_burn_rate;
///
/// assert_eq!(error_budget_burn_rate(21.6, 43.2), Some(0.5));
/// assert_eq!(error_budget_burn_rate(1.0, 0.0), None);
/// ```
#[must_use]
pub fn error_budget_burn_rate(actual_downtime_minutes: f64, budget_minutes: f64) -> Option<f64> {
    if budget_minutes == 0.0 {
        return None;
    }
    Some(actual_downtime_minutes / budget_minutes)
}

/// Whether the error budget is exhausted: burn rate at or above `1.0`.
///
/// # Arguments
///
/// * `actual_downtime_minutes` — actual downtime observed in the period, in
///   minutes.
/// * `budget_minutes` — the allowed downtime for the period, in minutes.
///
/// # Returns
///
/// `Some(true)` if the budget is exhausted or overspent, `Some(false)`
/// otherwise, or `None` if `budget_minutes` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::error_budget::is_error_budget_exhausted;
///
/// assert_eq!(is_error_budget_exhausted(50.0, 43.2), Some(true));
/// assert_eq!(is_error_budget_exhausted(20.0, 43.2), Some(false));
/// assert_eq!(is_error_budget_exhausted(1.0, 0.0), None);
/// ```
#[must_use]
pub fn is_error_budget_exhausted(actual_downtime_minutes: f64, budget_minutes: f64) -> Option<bool> {
    let burn_rate = error_budget_burn_rate(actual_downtime_minutes, budget_minutes)?;
    Some(burn_rate >= 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // "a 99.9% availability target over 30 days permits roughly 43 minutes
    // of allowed downtime."
    #[test]
    fn error_budget_matches_the_ninety_nine_point_nine_percent_worked_example() {
        let budget = error_budget_minutes(99.9, 30.0);
        assert!((budget - 43.2).abs() < 1e-9);
    }

    #[test]
    fn error_budget_burn_rate_computes_proportion_spent() {
        let burn_rate = error_budget_burn_rate(21.6, 43.2).unwrap();
        assert!((burn_rate - 0.5).abs() < 1e-9);
    }

    #[test]
    fn error_budget_burn_rate_is_none_for_zero_budget() {
        assert_eq!(error_budget_burn_rate(1.0, 0.0), None);
    }

    // "Agree, in advance ... what happens when the budget is exhausted."
    #[test]
    fn budget_is_exhausted_once_burn_rate_reaches_one() {
        assert_eq!(is_error_budget_exhausted(50.0, 43.2), Some(true));
        assert_eq!(is_error_budget_exhausted(20.0, 43.2), Some(false));
    }

    #[test]
    fn budget_exhaustion_is_none_for_zero_budget() {
        assert_eq!(is_error_budget_exhausted(1.0, 0.0), None);
    }
}

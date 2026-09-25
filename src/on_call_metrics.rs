//! # On-Call, Capacity, and Operational Load Metrics
//!
//! On-call load often concentrates on a small number of experienced people
//! who can resolve incidents fastest — the same pattern this book warns
//! against for code review load and knowledge concentration elsewhere,
//! applied here to operational burden. A team-wide average page frequency
//! can hide this concentration entirely; measuring the busiest individual's
//! share, and how often any one person is on call relative to a sustainable
//! limit, surfaces the burnout and bus-factor risk a simple average cannot.
//!
//! ## Formula
//!
//! ```text
//! Paging concentration (%)      = busiest engineer's pages / total pages × 100
//! On-call frequency ratio        = weeks on call / total weeks
//! Exceeds sustainable frequency   when on-call frequency ratio > max ratio
//!                                   (commonly 0.25, "no more than one week in four")
//! ```
//!
//! ## Why it matters
//!
//! Rebalancing rotations deliberately, once concentration appears, depends
//! on actually measuring individual-level page distribution rather than
//! only a team-wide average — the average can look entirely reasonable
//! while two or three people effectively carry the rotation due to skill
//! gaps or availability constraints. Aggregate this data at the team level
//! to inform staffing and hiring decisions; never use individual
//! page-response metrics to evaluate a specific engineer's performance.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::on_call_metrics::{
//!     paging_concentration_percent, on_call_frequency_ratio,
//!     exceeds_sustainable_on_call_frequency,
//! };
//!
//! // Of 40 pages across the team last quarter, the busiest engineer took 22.
//! let concentration = paging_concentration_percent(22.0, 40.0).unwrap();
//! assert!((concentration - 55.0).abs() < 1e-9);
//!
//! // That same engineer was on call 6 of the last 12 weeks: one week in two,
//! // well past the "no more than one week in four or five" guideline.
//! let ratio = on_call_frequency_ratio(6.0, 12.0).unwrap();
//! assert_eq!(ratio, 0.5);
//! assert_eq!(exceeds_sustainable_on_call_frequency(6.0, 12.0, 0.25), Some(true));
//! ```
//!
//! ## Pitfalls
//!
//! - **Reporting only a team-wide average page frequency** — hides severe
//!   individual concentration that drives both burnout and bus-factor risk.
//! - **Treating a nominally adequate rotation roster as sufficient** without
//!   checking whether it effectively relies on only two or three people due
//!   to skill gaps or availability constraints.
//! - **Measuring only active incident time**, ignoring the psychological
//!   cost of being on call even during a shift with zero pages.
//! - **Using individual page-response metrics to evaluate a specific
//!   engineer** — the goal is sustainable staffing and system design, never
//!   individual scorekeeping.
//!
//! ## Sources
//!
//! - Chapter 6.3, On-call, capacity, and operational load metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/06-03-on-call-capacity-and-operational-load-metrics.md

/// Paging concentration: how much of the team's total paging load fell on
/// the single busiest on-call engineer.
///
/// `busiest_engineer_pages / total_pages × 100`.
///
/// # Arguments
///
/// * `busiest_engineer_pages` — count of pages received by the busiest
///   individual engineer.
/// * `total_pages` — total count of pages received by the whole team.
///
/// # Returns
///
/// The concentration as a percentage, or `None` if `total_pages` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::on_call_metrics::paging_concentration_percent;
///
/// assert!((paging_concentration_percent(22.0, 40.0).unwrap() - 55.0).abs() < 1e-9);
/// assert_eq!(paging_concentration_percent(1.0, 0.0), None);
/// ```
#[must_use]
pub fn paging_concentration_percent(busiest_engineer_pages: f64, total_pages: f64) -> Option<f64> {
    if total_pages == 0.0 {
        return None;
    }
    Some((busiest_engineer_pages / total_pages) * 100.0)
}

/// On-call frequency ratio: the fraction of weeks an engineer spent on
/// call.
///
/// `weeks_on_call / total_weeks`.
///
/// # Arguments
///
/// * `weeks_on_call` — count of weeks the engineer was on call.
/// * `total_weeks` — total number of weeks in the observation period.
///
/// # Returns
///
/// The ratio, or `None` if `total_weeks` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::on_call_metrics::on_call_frequency_ratio;
///
/// assert_eq!(on_call_frequency_ratio(6.0, 12.0), Some(0.5));
/// assert_eq!(on_call_frequency_ratio(1.0, 0.0), None);
/// ```
#[must_use]
pub fn on_call_frequency_ratio(weeks_on_call: f64, total_weeks: f64) -> Option<f64> {
    if total_weeks == 0.0 {
        return None;
    }
    Some(weeks_on_call / total_weeks)
}

/// Whether an engineer's on-call frequency exceeds a given sustainable
/// maximum, such as the chapter's example of "no more than one week in four
/// or five" (a `max_ratio` of `0.25` or `0.20`).
///
/// # Arguments
///
/// * `weeks_on_call` — count of weeks the engineer was on call.
/// * `total_weeks` — total number of weeks in the observation period.
/// * `max_ratio` — the maximum sustainable on-call frequency ratio.
///
/// # Returns
///
/// `Some(true)` if the engineer's ratio exceeds `max_ratio`, `Some(false)`
/// otherwise, or `None` if `total_weeks` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::on_call_metrics::exceeds_sustainable_on_call_frequency;
///
/// // 6 of 12 weeks is one week in two, well past a one-in-four limit.
/// assert_eq!(exceeds_sustainable_on_call_frequency(6.0, 12.0, 0.25), Some(true));
/// // 3 of 12 weeks is exactly one week in four: not exceeding it.
/// assert_eq!(exceeds_sustainable_on_call_frequency(3.0, 12.0, 0.25), Some(false));
/// ```
#[must_use]
pub fn exceeds_sustainable_on_call_frequency(
    weeks_on_call: f64,
    total_weeks: f64,
    max_ratio: f64,
) -> Option<bool> {
    let ratio = on_call_frequency_ratio(weeks_on_call, total_weeks)?;
    Some(ratio > max_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Measure how many pages each individual on-call engineer receives,
    // not just a team-wide average that can hide severe concentration."
    #[test]
    fn paging_concentration_computes_share_of_total_pages() {
        let concentration = paging_concentration_percent(22.0, 40.0).unwrap();
        assert!((concentration - 55.0).abs() < 1e-6);
    }

    #[test]
    fn paging_concentration_is_none_for_zero_total_pages() {
        assert_eq!(paging_concentration_percent(1.0, 0.0), None);
    }

    #[test]
    fn on_call_frequency_ratio_computes_fraction_of_weeks() {
        let ratio = on_call_frequency_ratio(6.0, 12.0).unwrap();
        assert!((ratio - 0.5).abs() < 1e-9);
    }

    #[test]
    fn on_call_frequency_ratio_is_none_for_zero_total_weeks() {
        assert_eq!(on_call_frequency_ratio(1.0, 0.0), None);
    }

    // "Establish a maximum reasonable frequency for how often any
    // individual should be on call, commonly no more than one week in four
    // or five."
    #[test]
    fn frequency_exceeding_one_week_in_four_is_flagged() {
        assert_eq!(exceeds_sustainable_on_call_frequency(6.0, 12.0, 0.25), Some(true));
    }

    #[test]
    fn frequency_at_exactly_one_week_in_four_does_not_exceed_it() {
        assert_eq!(exceeds_sustainable_on_call_frequency(3.0, 12.0, 0.25), Some(false));
    }

    #[test]
    fn exceeds_sustainable_frequency_is_none_for_zero_total_weeks() {
        assert_eq!(exceeds_sustainable_on_call_frequency(1.0, 0.0, 0.25), None);
    }
}

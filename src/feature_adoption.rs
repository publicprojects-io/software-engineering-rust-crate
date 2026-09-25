//! # Feature Adoption and Usage Metrics
//!
//! **Feature adoption** measures whether the people a feature was built for
//! actually use it, at what rate, and whether that use persists over time.
//! Initial adoption and sustained adoption are different signals: a spike
//! from curiosity or forced exposure is not the same as genuine, lasting
//! value delivery. Track them separately, and measure both against the
//! specific target audience the feature was built for, not your entire user
//! base indiscriminately.
//!
//! ## Formula
//!
//! ```text
//! Initial adoption %  = tried_at_least_once / target_audience × 100
//! Retained adoption % = still_using_after_n_weeks / initially_tried × 100
//!
//! tried_at_least_once     = people in the target audience who tried the
//!                            feature at least once
//! target_audience         = the specific population the feature was built
//!                            for (not the whole user base)
//! still_using_after_n_weeks = of those initial triers, how many are still
//!                            using the feature after a meaningful period
//!                            (e.g. four or eight weeks)
//! initially_tried          = the initial-trial count (the denominator for
//!                            retention, distinct from target_audience)
//! ```
//!
//! ## Why it matters
//!
//! A feature with high initial trial and low retention suggests
//! discoverability worked but the feature itself did not deliver enough
//! value to keep people coming back — a very different diagnosis, and a
//! very different fix, than low initial trial with high retention, which
//! suggests a genuinely valuable feature that not enough people know about.
//! Reporting only one of the two numbers hides exactly this distinction.
//!
//! ## Example
//!
//! The topic doc's enterprise example: a collaborative-editing feature
//! launch reported "an impressive 60% initial trial rate within the first
//! two weeks," but "a follow-up retention read at eight weeks showed only
//! 8% of those initial triers were still using the feature regularly" —
//! revealing the high trial rate had been driven by a hard-to-dismiss
//! onboarding tooltip rather than genuine, sustained interest.
//!
//! ```rust
//! use software_engineering::feature_adoption::{
//!     initial_adoption_percent, retained_adoption_percent,
//! };
//!
//! // 600 of a 1,000-person target audience tried the feature: 60% initial trial.
//! let initial = initial_adoption_percent(600.0, 1_000.0).unwrap();
//! assert!((initial - 60.0).abs() < 1e-9);
//!
//! // Of those 600 initial triers, only 48 (8%) were still using it at 8 weeks.
//! let retained = retained_adoption_percent(48.0, 600.0).unwrap();
//! assert!((retained - 8.0).abs() < 1e-9);
//! ```
//!
//! ## Pitfalls
//!
//! - **Reporting only initial trial, never retention** — cannot distinguish
//!   curiosity or forced exposure from genuine, lasting value.
//! - **Measuring adoption against the wrong denominator** — a feature built
//!   for a specific segment, measured against the whole user base, will
//!   always look like it has terrible adoption regardless of how well it
//!   actually serves its intended audience.
//! - **Concluding a feature failed without investigating the specific
//!   cause** of low adoption — it may be poorly discovered, poorly
//!   explained, or simply too new for the measurement window.
//! - **Celebrating adoption inflated by forced exposure or dark patterns**
//!   — a hard-to-dismiss modal or intrusive default is not genuine, voluntary use.
//!
//! ## Sources
//!
//! - Chapter 5.2, Feature adoption and usage metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/05-02-feature-adoption-and-usage-metrics.md

/// Initial adoption percentage: target audience who tried a feature at
/// least once.
///
/// Measure against the feature's specific, precisely defined target
/// audience, not your entire user base — a feature for enterprise
/// administrators measured against a mostly individual-user base will
/// always look like it has terrible adoption, regardless of how well it
/// actually serves the people it was built for.
///
/// # Arguments
///
/// * `tried_at_least_once` — count of the target audience who tried the
///   feature at least once.
/// * `target_audience` — size of the specific population the feature was
///   built for.
///
/// # Returns
///
/// `Some(percentage)` (e.g. `60.0` for 60%), or `None` when
/// `target_audience` is zero (no audience — adoption undefined).
///
/// # Examples
///
/// ```rust
/// use software_engineering::feature_adoption::initial_adoption_percent;
///
/// // 600 of 1,000 in the target audience tried the feature: 60% initial trial.
/// assert_eq!(initial_adoption_percent(600.0, 1_000.0), Some(60.0));
/// assert_eq!(initial_adoption_percent(1.0, 0.0), None);
/// ```
#[must_use]
pub fn initial_adoption_percent(
    tried_at_least_once: f64,
    target_audience: f64,
) -> Option<f64> {
    if target_audience == 0.0 {
        None
    } else {
        Some(tried_at_least_once / target_audience * 100.0)
    }
}

/// Retained adoption percentage: initial triers still using the feature
/// after a meaningful period (e.g. four or eight weeks).
///
/// A feature with high initial trial and low retention suggests
/// discoverability worked but the feature itself did not deliver enough
/// value to keep people coming back. Always report this alongside initial
/// adoption, never in isolation.
///
/// # Arguments
///
/// * `still_using_after_n_weeks` — count of initial triers still using the
///   feature after the chosen follow-up period.
/// * `initially_tried` — count of people who tried the feature at least
///   once (the retention denominator).
///
/// # Returns
///
/// `Some(percentage)` (e.g. `8.0` for 8%), or `None` when `initially_tried`
/// is zero (no initial triers — retention undefined).
///
/// # Examples
///
/// ```rust
/// use software_engineering::feature_adoption::retained_adoption_percent;
///
/// // Only 48 of 600 initial triers (8%) were still using it at 8 weeks.
/// assert_eq!(retained_adoption_percent(48.0, 600.0), Some(8.0));
/// assert_eq!(retained_adoption_percent(1.0, 0.0), None);
/// ```
#[must_use]
pub fn retained_adoption_percent(
    still_using_after_n_weeks: f64,
    initially_tried: f64,
) -> Option<f64> {
    if initially_tried == 0.0 {
        None
    } else {
        Some(still_using_after_n_weeks / initially_tried * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // "an impressive 60% initial trial rate within the first two weeks."
    #[test]
    fn collaborative_editing_feature_had_60_percent_initial_trial() {
        let initial = initial_adoption_percent(600.0, 1_000.0).unwrap();
        assert!((initial - 60.0).abs() < 1e-9);
    }

    // "a follow-up retention read at eight weeks showed only 8% of those
    // initial triers were still using the feature regularly."
    #[test]
    fn only_8_percent_of_initial_triers_were_retained_at_8_weeks() {
        let retained = retained_adoption_percent(48.0, 600.0).unwrap();
        assert!((retained - 8.0).abs() < 1e-9);
    }

    // "Adoption should be measured against the audience it was built for,
    // not against your entire user base indiscriminately" — a zero-size
    // audience or zero initial triers leaves the rate undefined.
    #[test]
    fn adoption_is_undefined_with_a_zero_denominator() {
        assert!(initial_adoption_percent(10.0, 0.0).is_none());
        assert!(retained_adoption_percent(10.0, 0.0).is_none());
    }
}

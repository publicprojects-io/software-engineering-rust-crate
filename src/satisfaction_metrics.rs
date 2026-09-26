//! # Satisfaction and Well-Being Metrics
//!
//! **Satisfaction and well-being**, the S in SPACE (chapter 3.1), is the
//! dimension no system telemetry can observe directly. Whether an
//! engineer finds their work meaningful, whether they feel supported by
//! their team, whether they are heading toward burnout — none of this
//! leaves a trace in a version control log or a CI pipeline. It has to
//! be asked, deliberately and well, with genuine anonymity as
//! non-negotiable. This dimension is a leading indicator: declining
//! satisfaction predicts attrition before an exit interview does, and
//! rising burnout risk predicts a quality collapse before the defect
//! rate shows it.
//!
//! ## Formula
//!
//! ```text
//! Satisfaction net score = ((promoters - detractors) / total_respondents) × 100
//!     (an employee Net Promoter-style score, ranging roughly -100 to +100)
//!
//! Declining  when current_score < previous_score - decline_threshold
//! ```
//!
//! ## Why it matters
//!
//! Any perceived link between an honest answer and a personal
//! consequence destroys the signal almost immediately: satisfaction data
//! used to understand and improve team conditions is valuable and
//! low-risk, but the same data used to rank teams or, worse,
//! individuals against each other corrupts the survey instrument the
//! moment people suspect the answer will be used against them or their
//! team. This module computes an aggregate score and a trend signal only
//! — it has no concept of an individual respondent, and callers must
//! guarantee genuine anonymity in how they collect the inputs.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::satisfaction_metrics::{
//!     satisfaction_net_score, is_satisfaction_declining,
//! };
//!
//! // Of 50 respondents, 30 are promoters and 10 are detractors: a net
//! // score of +40.
//! let score = satisfaction_net_score(30.0, 10.0, 50.0).unwrap();
//! assert!((score - 40.0).abs() < 1e-9);
//!
//! // A drop from +40 to +15 (25 points) past a 10-point threshold is a
//! // leading-indicator warning worth investigating before it shows up
//! // as attrition.
//! assert!(is_satisfaction_declining(40.0, 15.0, 10.0));
//! ```
//!
//! ## Pitfalls
//!
//! - **Breaking anonymity, even accidentally** — a single incident where
//!   individual responses can be traced back to a person destroys trust
//!   in every future survey, especially in small teams where response
//!   patterns could otherwise be inferable.
//! - **Using satisfaction data to rank teams or individuals** — corrupts
//!   the signal almost immediately once people suspect the answer will
//!   be used against them.
//! - **Reading a single reading in isolation** — this dimension is a
//!   leading indicator; track the trend over time, not one snapshot.
//! - **Ad hoc, unvalidated survey questions** — produces data of unclear
//!   meaning that resists honest interpretation.
//!
//! ## Sources
//!
//! - Chapter 3.2, Satisfaction and well-being metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/03-02-satisfaction-and-well-being-metrics.md

/// Employee-satisfaction Net Promoter-style score: the percentage of
/// promoters minus the percentage of detractors among survey
/// respondents.
///
/// `((promoters - detractors) / total_respondents) × 100`. Ranges
/// roughly from -100 (every respondent a detractor) to +100 (every
/// respondent a promoter). This aggregate score has no concept of any
/// individual respondent — genuine anonymity in collecting the inputs is
/// the caller's responsibility, per the chapter's central recommendation.
///
/// # Arguments
///
/// * `promoters` — count of respondents classified as promoters.
/// * `detractors` — count of respondents classified as detractors.
/// * `total_respondents` — total count of respondents (promoters,
///   passives, and detractors combined).
///
/// # Returns
///
/// The net score, or `None` if `total_respondents` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::satisfaction_metrics::satisfaction_net_score;
///
/// // 30 promoters, 10 detractors, out of 50 respondents: net +40.
/// assert_eq!(satisfaction_net_score(30.0, 10.0, 50.0), Some(40.0));
/// // More detractors than promoters yields a negative score.
/// assert_eq!(satisfaction_net_score(5.0, 20.0, 50.0), Some(-30.0));
/// assert_eq!(satisfaction_net_score(1.0, 1.0, 0.0), None);
/// ```
#[must_use]
pub fn satisfaction_net_score(promoters: f64, detractors: f64, total_respondents: f64) -> Option<f64> {
    if total_respondents == 0.0 {
        return None;
    }
    Some(((promoters - detractors) / total_respondents) * 100.0)
}

/// Whether a satisfaction score has declined enough between two
/// measurement periods to warrant treating it as an early
/// attrition/burnout warning, per the chapter's framing of this
/// dimension as a leading indicator rather than a lagging one.
///
/// True iff `current_score < previous_score - decline_threshold`.
///
/// # Arguments
///
/// * `previous_score` — the satisfaction net score from an earlier
///   measurement period.
/// * `current_score` — the satisfaction net score from the current
///   period.
/// * `decline_threshold` — how many points of decline (a positive
///   number) is treated as meaningful rather than ordinary noise.
///
/// # Returns
///
/// `true` if the decline exceeds `decline_threshold`.
///
/// # Examples
///
/// ```rust
/// use software_engineering::satisfaction_metrics::is_satisfaction_declining;
///
/// // A 25-point drop past a 10-point threshold: a real warning.
/// assert!(is_satisfaction_declining(40.0, 15.0, 10.0));
/// // A 3-point drop within a 10-point threshold: ordinary noise.
/// assert!(!is_satisfaction_declining(40.0, 37.0, 10.0));
/// // A rise is never a decline.
/// assert!(!is_satisfaction_declining(40.0, 55.0, 10.0));
/// ```
#[must_use]
pub fn is_satisfaction_declining(previous_score: f64, current_score: f64, decline_threshold: f64) -> bool {
    current_score < previous_score - decline_threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Whether an engineer finds their work meaningful... has to be
    // asked" — the net score aggregates exactly that survey data.
    #[test]
    fn net_score_is_positive_when_promoters_outnumber_detractors() {
        let score = satisfaction_net_score(30.0, 10.0, 50.0).unwrap();
        assert!((score - 40.0).abs() < 1e-9);
    }

    #[test]
    fn net_score_is_negative_when_detractors_outnumber_promoters() {
        let score = satisfaction_net_score(5.0, 20.0, 50.0).unwrap();
        assert!((score - (-30.0)).abs() < 1e-9);
    }

    #[test]
    fn net_score_is_none_for_zero_respondents() {
        assert_eq!(satisfaction_net_score(1.0, 1.0, 0.0), None);
    }

    // "This dimension is a leading indicator, not a lagging one. It
    // predicts [attrition and burnout] before they show up elsewhere."
    #[test]
    fn a_drop_past_the_threshold_is_flagged_as_declining() {
        assert!(is_satisfaction_declining(40.0, 15.0, 10.0));
    }

    #[test]
    fn a_small_drop_within_the_threshold_is_not_flagged() {
        assert!(!is_satisfaction_declining(40.0, 37.0, 10.0));
    }

    #[test]
    fn a_rise_is_never_flagged_as_declining() {
        assert!(!is_satisfaction_declining(40.0, 55.0, 10.0));
    }
}

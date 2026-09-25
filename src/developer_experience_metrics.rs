//! # Developer Experience Metrics
//!
//! Two practical measures from Part 3 of the book: **focus time**, the
//! count and duration of uninterrupted two-hour-plus blocks per week
//! (chapter 3.6's efficiency-and-flow dimension), and **survey response
//! rate**, itself a trust signal for a DevEx survey programme, not merely a
//! data-collection statistic (chapter 3.7).
//!
//! ## Formula
//!
//! ```text
//! Focus block   = a calendar block >= 2.0 hours, uninterrupted
//! Response rate = (survey responses received / survey invitations sent) x 100%
//! ```
//!
//! ## Why it matters
//!
//! Refocusing after an interruption to deep, complex work routinely takes
//! many minutes, sometimes closer to half an hour, to fully rebuild the
//! working memory an engineer was holding before the interruption. An
//! engineer whose day is fragmented into short blocks may show plenty of
//! activity while accomplishing far less genuinely difficult work than the
//! same engineer would with two protected, uninterrupted hours. Separately,
//! a declining survey response rate often indicates eroding trust in the
//! process — survey fatigue, doubts that results lead to action, or
//! suspicion that anonymity is not genuinely protected — and deserves direct
//! investigation rather than being dismissed as a data-collection
//! inconvenience.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::developer_experience_metrics::{
//!     is_focus_block, response_rate_percent,
//! };
//!
//! // "Uninterrupted blocks of two hours or more" count as focus time.
//! assert!(is_focus_block(2.0));
//! assert!(is_focus_block(2.5));
//! assert!(!is_focus_block(1.75));
//!
//! // A DevEx survey sent to 100 engineers, 72 responses: 72% response rate.
//! assert_eq!(response_rate_percent(72.0, 100.0), Some(72.0));
//! ```
//!
//! ## Pitfalls
//!
//! - **Using interruption or notification data as individual surveillance**
//!   repeats the misuse risk the book warns against for activity data;
//!   aggregate at the team level.
//! - **Imposing a single, rigid focus-time schedule on everyone** ignores
//!   genuine individual variation in how people work best.
//! - **Ignoring a declining response rate** misses an important trust
//!   signal in its own right — investigate rather than dismiss it.
//!
//! ## Sources
//!
//! - Chapter 3.6, Efficiency and flow: deep work and interruptions.
//! - Chapter 3.7, Developer experience surveys and DevEx metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/03-06-efficiency-and-flow.md
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/03-07-developer-experience-surveys-and-devex-metrics.md

/// Whether a calendar block qualifies as protected "focus time".
///
/// The book defines focus time as "uninterrupted blocks of two hours or
/// more" measured from calendar data.
///
/// # Arguments
///
/// * `duration_hours` — length of the uninterrupted calendar block, in
///   hours.
///
/// # Returns
///
/// `true` iff `duration_hours >= 2.0`.
///
/// # Examples
///
/// ```rust
/// use software_engineering::developer_experience_metrics::is_focus_block;
///
/// assert!(is_focus_block(2.0));
/// assert!(!is_focus_block(1.99));
/// ```
pub fn is_focus_block(duration_hours: f64) -> bool {
    duration_hours >= 2.0
}

/// Survey response rate as a percentage: responses received / invitations
/// sent x 100.
///
/// Treat this as a diagnostic signal in its own right (chapter 3.7): a
/// declining rate often indicates eroding trust in the survey process.
///
/// # Arguments
///
/// * `responses_received` — count of survey responses received.
/// * `invitations_sent` — count of survey invitations sent.
///
/// # Returns
///
/// `Some(percentage)`, or `None` when `invitations_sent` is zero (rate
/// undefined).
///
/// # Examples
///
/// ```rust
/// use software_engineering::developer_experience_metrics::response_rate_percent;
///
/// assert_eq!(response_rate_percent(72.0, 100.0), Some(72.0));
/// assert_eq!(response_rate_percent(1.0, 0.0), None);
/// ```
pub fn response_rate_percent(responses_received: f64, invitations_sent: f64) -> Option<f64> {
    if invitations_sent == 0.0 {
        None
    } else {
        Some(responses_received / invitations_sent * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Calculate the number and duration of uninterrupted blocks of two
    // hours or more available in an engineer's typical week."
    #[test]
    fn two_hour_block_is_a_focus_block() {
        assert!(is_focus_block(2.0));
        assert!(is_focus_block(2.5));
    }

    // A block shorter than two hours does not count as focus time.
    #[test]
    fn sub_two_hour_block_is_not_a_focus_block() {
        assert!(!is_focus_block(1.75));
        assert!(!is_focus_block(0.0));
    }

    // "Response rate rose to over 70% within two cycles" — a percentage
    // computed the same way as this function.
    #[test]
    fn response_rate_computes_percentage() {
        assert!((response_rate_percent(72.0, 100.0).unwrap() - 72.0).abs() < 1e-9);
        assert!(response_rate_percent(1.0, 0.0).is_none());
    }

    // "A software company's initial DevEx survey included [an] under 30%"
    // response rate — verifying the formula against that figure.
    #[test]
    fn response_rate_matches_under_30_percent_example() {
        let rate = response_rate_percent(29.0, 100.0).unwrap();
        assert!(rate < 30.0);
    }
}

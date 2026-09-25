//! # Pull Request and Code Review Metrics
//!
//! Code review is usually the single largest wait-time contributor inside
//! the cycle-time breakdown, and it is also the stage most directly under a
//! team's own control to improve. This module covers two of that stage's
//! core metrics: **time to first review**, the dominant wait-time lever, and
//! **reviewer load concentration**, a way to surface an otherwise-invisible
//! bottleneck and bus-factor risk in who does the reviewing.
//!
//! ## Formula
//!
//! ```text
//! Time to first review        = t(first substantive comment or approval) - t(opened)
//! Reviewer load concentration = max(reviews per reviewer) / mean(reviews per reviewer)
//! ```
//!
//! ## Why it matters
//!
//! Most delay in the review stage comes from a pull request waiting to be
//! looked at, not from the review conversation taking long once it starts —
//! which is why time to first review, instrumented automatically from the
//! version control platform, "typically produces the largest single
//! improvement to overall cycle time available to a team." Separately,
//! reviewer load is commonly concentrated on a small number of people
//! without anyone measuring it directly: an enterprise example in the book
//! found "a handful of principal engineers were completing over 40% of all
//! code reviews across a two-hundred-person organization." That
//! concentration is both a bottleneck, since those engineers' availability
//! caps the whole team's review throughput, and a burnout risk.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::pull_request_metrics::{
//!     time_to_first_review, reviewer_load_concentration_ratio,
//! };
//!
//! // A pull request opened at hour 0 gets its first review comment at hour 5.
//! assert_eq!(time_to_first_review(0.0, 5.0), 5.0);
//!
//! // Five reviewers complete 40, 10, 10, 10, and 10 reviews in a quarter:
//! // one reviewer is doing 4x the average review load.
//! let reviews = [40.0, 10.0, 10.0, 10.0, 10.0];
//! let ratio = reviewer_load_concentration_ratio(&reviews).unwrap();
//! assert!((ratio - 4.0).abs() < 1e-9);
//! ```
//!
//! ## Pitfalls
//!
//! - **Optimizing time to first review without a paired quality guardrail**
//!   invites rubber-stamp approval that defeats review's purpose; a fast
//!   approval with no real scrutiny is worse than a slower, genuine one.
//! - **Reviewer load concentration is a diagnostic system signal for
//!   spotting bottleneck and bus-factor risk — never an individual
//!   performance scorecard.** The book is explicit that review-related
//!   counts are "more often a system or communication signal than a
//!   personal one," and warns directly against "the evaluative drift"
//!   of treating them as a judgement on any one reviewer or author. Use a
//!   high ratio to prompt rotation and knowledge-sharing, not to rank or
//!   evaluate the individuals involved.
//! - **Ignoring reviewer load concentration** leaves it invisible until it
//!   surfaces as a bottleneck (the concentrated reviewers' availability caps
//!   throughput) or a burnout event.
//!
//! ## Sources
//!
//! - Chapter 2.9, Pull request and code review metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/02-09-pull-request-and-code-review-metrics.md

/// Time to first review: the interval from a pull request being opened to a
/// reviewer's first substantive comment or approval.
///
/// The book identifies this as "usually the dominant wait-time contributor"
/// within the review stage, and improving it typically produces the largest
/// single improvement to overall cycle time available to a team.
///
/// # Arguments
///
/// * `opened_at` — the time the pull request was opened (any consistent time
///   unit, e.g. hours since epoch).
/// * `first_response_at` — the time of the reviewer's first substantive
///   comment or approval, in the same unit.
///
/// # Returns
///
/// The elapsed time between opening and first review response, in the same
/// unit as the inputs.
///
/// # Examples
///
/// ```rust
/// use software_engineering::pull_request_metrics::time_to_first_review;
///
/// // Opened at hour 10, first reviewed at hour 34: an 18-hour wait.
/// assert_eq!(time_to_first_review(10.0, 34.0), 24.0);
/// ```
pub fn time_to_first_review(opened_at: f64, first_response_at: f64) -> f64 {
    first_response_at - opened_at
}

/// Reviewer load concentration ratio: the busiest reviewer's review count
/// divided by the mean review count across all reviewers.
///
/// This is a **diagnostic system signal**, not an individual performance
/// scorecard. The book's own worked example, "a handful of principal
/// engineers were completing over 40% of all code reviews across a
/// two-hundred-person organization," is precisely the pattern this ratio is
/// meant to surface: a bottleneck (the concentrated reviewers' availability
/// caps team-wide review throughput) and a burnout risk, not evidence that
/// any individual reviewer is doing something wrong. Use a high ratio to
/// prompt review rotation and knowledge-sharing — never to rank or evaluate
/// individual reviewers.
///
/// # Arguments
///
/// * `reviews_per_reviewer` — completed review counts for each reviewer over
///   a rolling window.
///
/// # Returns
///
/// `Some(ratio)` where `ratio` is the maximum value divided by the mean of
/// `reviews_per_reviewer`; `None` when the slice is empty or the mean is
/// zero (ratio undefined). A ratio near `1.0` indicates evenly distributed
/// review load; a high ratio indicates concentration on a small number of
/// people.
///
/// # Examples
///
/// ```rust
/// use software_engineering::pull_request_metrics::reviewer_load_concentration_ratio;
///
/// // Evenly distributed load: ratio is 1.0.
/// let even = [10.0, 10.0, 10.0, 10.0];
/// assert!((reviewer_load_concentration_ratio(&even).unwrap() - 1.0).abs() < 1e-9);
///
/// // Concentrated load: one reviewer far above the mean.
/// let concentrated = [40.0, 10.0, 10.0, 10.0, 10.0];
/// assert!((reviewer_load_concentration_ratio(&concentrated).unwrap() - 4.0).abs() < 1e-9);
///
/// assert_eq!(reviewer_load_concentration_ratio(&[]), None);
/// ```
pub fn reviewer_load_concentration_ratio(reviews_per_reviewer: &[f64]) -> Option<f64> {
    if reviews_per_reviewer.is_empty() {
        return None;
    }
    let sum: f64 = reviews_per_reviewer.iter().sum();
    let mean = sum / reviews_per_reviewer.len() as f64;
    if mean == 0.0 {
        return None;
    }
    let max = reviews_per_reviewer
        .iter()
        .cloned()
        .fold(f64::MIN, f64::max);
    Some(max / mean)
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Measure the interval from a pull request being opened to a
    // reviewer's first substantive comment or approval."
    #[test]
    fn time_to_first_review_is_first_response_minus_opened() {
        assert_eq!(time_to_first_review(10.0, 34.0), 24.0);
        assert_eq!(time_to_first_review(0.0, 5.0), 5.0);
    }

    // Evenly distributed review load produces a concentration ratio of 1.0.
    #[test]
    fn even_review_load_has_ratio_of_one() {
        let even = [10.0, 10.0, 10.0, 10.0];
        assert!((reviewer_load_concentration_ratio(&even).unwrap() - 1.0).abs() < 1e-9);
    }

    // "A handful of principal engineers were completing over 40% of all
    // code reviews across a two-hundred-person organization" — a
    // worked example of concentrated review load, here as one reviewer
    // doing four times the average.
    #[test]
    fn concentrated_review_load_has_high_ratio() {
        let concentrated = [40.0, 10.0, 10.0, 10.0, 10.0];
        let ratio = reviewer_load_concentration_ratio(&concentrated).unwrap();
        assert!((ratio - 4.0).abs() < 1e-9);
    }

    // Undefined for an empty slice or an all-zero slice (mean is zero).
    #[test]
    fn ratio_is_none_for_empty_or_zero_mean_input() {
        assert_eq!(reviewer_load_concentration_ratio(&[]), None);
        assert_eq!(reviewer_load_concentration_ratio(&[0.0, 0.0, 0.0]), None);
    }
}

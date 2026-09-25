//! # Cycle Time and Its Components
//!
//! Cycle time is the internal breakdown of a change's flow time (see
//! [`crate::flow_framework`]) into its constituent engineering stages:
//! coding, pickup (waiting for a reviewer to start), review, test, and
//! deploy. Where flow time gives a single end-to-end number, cycle time
//! tells you where that time actually goes once work reaches engineering —
//! the diagnostic layer underneath the summary number.
//!
//! ## Formula
//!
//! ```text
//! Cycle time = coding + pickup + review + test + deploy
//!
//! coding  = first commit → pull request opened
//! pickup  = pull request opened → first review
//! review  = first review → approval
//! test    = time spent in automated/manual verification
//! deploy  = approval → production
//! ```
//!
//! ## Why it matters
//!
//! "Lead time is too long" is not actionable on its own. A team whose lead
//! time is dominated by coding time needs a different intervention than a
//! team whose lead time is dominated by a three-day review queue, which
//! needs a different intervention again than a team losing most of its time
//! to a flaky, slow test suite. Without cycle-time decomposition, teams
//! guess at the bottleneck, and the guess is wrong often enough that fixing
//! the wrong stage wastes real effort while the actual constraint stays
//! untouched. Decomposition by stage also reveals shared, cross-team
//! bottlenecks — for example a single overloaded shared review pool — that
//! no individual team's own metrics can see on their own.
//!
//! ## Example
//!
//! A change with 2 days of coding, 0.5 days waiting for pickup, 1.5 days in
//! review, 0.5 days in test, and 0.5 days to deploy has a cycle time of 5
//! days; review is 30% of the total, the largest single share.
//!
//! ```rust
//! use software_engineering::cycle_time::{cycle_time, stage_percent_of_cycle};
//!
//! let total = cycle_time(2.0, 0.5, 1.5, 0.5, 0.5);
//! assert_eq!(total, 5.0);
//!
//! let review_share = stage_percent_of_cycle(1.5, total).unwrap();
//! assert!((review_share - 30.0).abs() < 1e-9);
//! ```
//!
//! ## Pitfalls
//!
//! - **Reacting to a lead-time regression without cycle-time diagnosis**:
//!   frequently leads to fixing the wrong stage.
//! - **Assuming active effort, not wait time, is the dominant cost**:
//!   usually wrong — queueing dominates in most real delivery pipelines (see
//!   [`crate::flow_framework::flow_efficiency_percent`]).
//! - **Missing a shared, cross-team bottleneck** by reviewing cycle time
//!   team by team only, rather than aggregating across teams.
//! - **Stage-boundary definitional drift**: marking a stage "started" or
//!   "finished" earlier or later than its documented definition flatters a
//!   number without real improvement. Audit stage-boundary instrumentation
//!   periodically against its documented definition.
//!
//! ## Sources
//!
//! - Chapter 2.6, "Cycle time and its components."
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/02-06-cycle-time-and-its-components.md

/// Cycle time: the sum of a change's five named engineering stages.
///
/// # Arguments
///
/// * `coding` — first commit to pull request opened.
/// * `pickup` — pull request opened to first review.
/// * `review` — first review to approval.
/// * `test` — time spent in automated/manual verification.
/// * `deploy` — approval to production.
///
/// # Returns
///
/// The total cycle time, in whatever unit the stage durations are expressed.
///
/// # Examples
///
/// ```rust
/// use software_engineering::cycle_time::cycle_time;
///
/// let total = cycle_time(2.0, 0.5, 1.5, 0.5, 0.5);
/// assert_eq!(total, 5.0);
/// ```
#[must_use]
pub fn cycle_time(coding: f64, pickup: f64, review: f64, test: f64, deploy: f64) -> f64 {
    coding + pickup + review + test + deploy
}

/// A single stage's share of total cycle time, as a percentage.
///
/// Use this to set stage-specific improvement targets ("reduce median review
/// wait time from two days to four hours") rather than a vague overall
/// "reduce lead time by 20%" goal that gives a team no guidance on where to
/// focus.
///
/// # Arguments
///
/// * `stage_duration` — the duration of one stage.
/// * `total_cycle_time` — the total cycle time across all stages.
///
/// # Returns
///
/// `Some(percentage)` (e.g. `30.0` for 30%), or `None` when
/// `total_cycle_time` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::cycle_time::stage_percent_of_cycle;
///
/// let review_share = stage_percent_of_cycle(1.5, 5.0).unwrap();
/// assert!((review_share - 30.0).abs() < 1e-9);
/// assert_eq!(stage_percent_of_cycle(1.5, 0.0), None);
/// ```
#[must_use]
pub fn stage_percent_of_cycle(stage_duration: f64, total_cycle_time: f64) -> Option<f64> {
    if total_cycle_time == 0.0 {
        None
    } else {
        Some(stage_duration / total_cycle_time * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Cycle time is the internal breakdown of a change's flow time into its
    // constituent engineering stages: coding time, review time, testing
    // time, and deploy time... further split into pickup time."
    #[test]
    fn cycle_time_sums_the_five_named_stages() {
        let total = cycle_time(2.0, 0.5, 1.5, 0.5, 0.5);
        assert!((total - 5.0).abs() < 1e-9);
    }

    // Worked example: review is the largest single share of a 5-day cycle.
    #[test]
    fn review_stage_is_thirty_percent_of_cycle_time() {
        let total = cycle_time(2.0, 0.5, 1.5, 0.5, 0.5);
        let review_share = stage_percent_of_cycle(1.5, total).unwrap();
        assert!((review_share - 30.0).abs() < 1e-9);
        assert!(stage_percent_of_cycle(1.5, 0.0).is_none());
    }
}

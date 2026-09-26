//! # Activity Metrics and Their Limits
//!
//! **Activity**, the A in SPACE (chapter 3.1), counts the volume of
//! engineering work observable from system telemetry: commits, pull
//! requests opened, lines of code changed. It is the easiest SPACE
//! dimension to measure, because every one of these events is already
//! logged automatically — and that ease of measurement is exactly what
//! makes this dimension the most dangerous one to over-weight. Activity
//! measures motion, not value: a commit count does not distinguish
//! between a commit that solved a hard problem elegantly and a commit
//! that split one meaningful change into five to look more productive.
//!
//! ## Formula
//!
//! ```text
//! Commit substitution gaming signal =
//!     commit_count rose AND average_commit_size shrank
//!
//! Activity rate = commits / engineers / weeks
//!     (a contextual signal only, never a standalone productivity proxy)
//! ```
//!
//! ## Why it matters
//!
//! This is the single most historically misused metric family in
//! software engineering measurement. Once activity becomes an
//! incentivized individual metric, gaming follows almost immediately:
//! padding commits, splitting changes trivially, avoiding deep,
//! unglamorous work that produces few visible events. This module exists
//! to detect that specific gaming pattern and to compute an aggregate
//! rate for context — never to rank or score an individual. Commit
//! count, lines of code, and pull request count should never appear in
//! an individual performance review, a comparative ranking, or any
//! context where an engineer's compensation, standing, or reputation
//! depends on the number.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::activity_metrics::{
//!     is_commit_substitution_gaming_signal, commits_per_engineer_per_week,
//! };
//!
//! // A team's commit count rose 40% while average commit size fell by
//! // more than half — meaningful changes were likely split into many
//! // trivial ones to inflate the count.
//! assert!(is_commit_substitution_gaming_signal(50.0, 70.0, 120.0, 50.0));
//!
//! // Both count and size rising together is ordinary growth, not gaming.
//! assert!(!is_commit_substitution_gaming_signal(50.0, 70.0, 120.0, 140.0));
//!
//! // Read only in aggregate, alongside the other SPACE dimensions —
//! // never as a standalone verdict on any one person or team.
//! let rate = commits_per_engineer_per_week(120.0, 6.0, 4.0).unwrap();
//! assert!((rate - 5.0).abs() < 1e-9);
//! ```
//!
//! ## Pitfalls
//!
//! - **Ranking or evaluating individuals by raw activity counts** — the
//!   single hardest, most important rule this chapter states; the moment
//!   activity becomes an incentivized individual metric, gaming follows
//!   almost immediately.
//! - **Reading an activity number in isolation** — a sharp drop in
//!   team-level commit activity alongside a rise in satisfaction might
//!   mean the team finally had breathing room to pay down technical
//!   debt, a positive pattern that looks alarming without that context.
//! - **Treating raw volume as a quality-adjacent signal** — prefer size
//!   relative to review depth, or the ratio of new code to code removed,
//!   over a bare count.
//! - **Missing the substitution-gaming pattern** — rising frequency
//!   alongside sharply falling change size is the clearest sign activity
//!   is being inflated rather than genuinely increasing.
//!
//! ## Sources
//!
//! - Chapter 3.4, Activity metrics and their limits.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/03-04-activity-metrics-and-their-limits.md

/// Whether a change in commit count and average commit size matches the
/// chapter's named substitution-gaming pattern: splitting genuinely
/// meaningful work into many small, trivial commits to inflate a count.
///
/// True iff `commit_count_after > commit_count_before` (the count rose)
/// **and** `average_commit_size_after < average_commit_size_before` (the
/// average size shrank). Either condition alone is ordinary variation;
/// both together are the specific pattern this chapter warns about.
///
/// # Arguments
///
/// * `commit_count_before` — commit count in the earlier period.
/// * `commit_count_after` — commit count in the later period.
/// * `average_commit_size_before` — average commit size (e.g. lines
///   changed) in the earlier period.
/// * `average_commit_size_after` — average commit size in the later
///   period.
///
/// # Returns
///
/// `true` if the count rose while the average size shrank.
///
/// # Examples
///
/// ```rust
/// use software_engineering::activity_metrics::is_commit_substitution_gaming_signal;
///
/// // Count up, size down: the gaming pattern.
/// assert!(is_commit_substitution_gaming_signal(50.0, 70.0, 120.0, 50.0));
///
/// // Count up, size also up: ordinary growth, not gaming.
/// assert!(!is_commit_substitution_gaming_signal(50.0, 70.0, 120.0, 140.0));
///
/// // Count down: not the gaming pattern, regardless of size.
/// assert!(!is_commit_substitution_gaming_signal(70.0, 50.0, 50.0, 120.0));
/// ```
#[must_use]
pub fn is_commit_substitution_gaming_signal(
    commit_count_before: f64,
    commit_count_after: f64,
    average_commit_size_before: f64,
    average_commit_size_after: f64,
) -> bool {
    commit_count_after > commit_count_before && average_commit_size_after < average_commit_size_before
}

/// Commits per engineer per week — a simple activity rate, provided only
/// as a contextual signal to read alongside the other SPACE dimensions,
/// never as a standalone productivity proxy or an individual ranking.
///
/// `commits / engineers / weeks`.
///
/// # Arguments
///
/// * `commits` — total commit count across the period.
/// * `engineers` — number of engineers the commits are spread across.
/// * `weeks` — length of the observation period, in weeks.
///
/// # Returns
///
/// The commit rate per engineer per week, or `None` if `engineers` or
/// `weeks` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::activity_metrics::commits_per_engineer_per_week;
///
/// // 120 commits across 6 engineers over 4 weeks: 5 commits/engineer/week.
/// assert_eq!(commits_per_engineer_per_week(120.0, 6.0, 4.0), Some(5.0));
/// assert_eq!(commits_per_engineer_per_week(120.0, 0.0, 4.0), None);
/// assert_eq!(commits_per_engineer_per_week(120.0, 6.0, 0.0), None);
/// ```
#[must_use]
pub fn commits_per_engineer_per_week(commits: f64, engineers: f64, weeks: f64) -> Option<f64> {
    if engineers == 0.0 || weeks == 0.0 {
        return None;
    }
    Some(commits / engineers / weeks)
}

#[cfg(test)]
mod tests {
    use super::*;

    // "The most common way activity metrics get gamed is exactly chapter
    // 1.2's substitution pattern: splitting genuinely meaningful work
    // into many small, trivial events to inflate a count."
    #[test]
    fn rising_count_with_shrinking_size_is_a_gaming_signal() {
        assert!(is_commit_substitution_gaming_signal(50.0, 70.0, 120.0, 50.0));
    }

    #[test]
    fn rising_count_with_rising_size_is_not_a_gaming_signal() {
        assert!(!is_commit_substitution_gaming_signal(50.0, 70.0, 120.0, 140.0));
    }

    #[test]
    fn falling_count_is_not_a_gaming_signal_regardless_of_size() {
        assert!(!is_commit_substitution_gaming_signal(70.0, 50.0, 50.0, 120.0));
    }

    // "Activity data becomes genuinely useful when aggregated at the
    // team level and read alongside the other SPACE dimensions."
    #[test]
    fn commit_rate_divides_across_engineers_and_weeks() {
        let rate = commits_per_engineer_per_week(120.0, 6.0, 4.0).unwrap();
        assert!((rate - 5.0).abs() < 1e-9);
    }

    #[test]
    fn commit_rate_is_none_for_zero_engineers() {
        assert_eq!(commits_per_engineer_per_week(120.0, 0.0, 4.0), None);
    }

    #[test]
    fn commit_rate_is_none_for_zero_weeks() {
        assert_eq!(commits_per_engineer_per_week(120.0, 6.0, 0.0), None);
    }
}

//! # The SPACE Framework
//!
//! SPACE measures developer productivity across five dimensions instead of
//! one: **Satisfaction and well-being**, **Performance**, **Activity**,
//! **Communication and collaboration**, and **Efficiency and flow**. No
//! single dimension is meant to stand alone; the framework's real
//! contribution is the discipline of holding all five in view together, so a
//! team cannot look productive on one axis while quietly damaging another.
//!
//! ## Formula
//!
//! ```text
//! SPACE dimensions (five, not a computed ratio):
//!   S = Satisfaction and well-being
//!   P = Performance
//!   A = Activity
//!   C = Communication and collaboration
//!   E = Efficiency and flow
//!
//! Minimum balanced composition:
//!   at least one metric from at least 3 of the 5 dimensions,
//!   mixing objective instrumentation with subjective survey data
//! ```
//!
//! ## Why it matters
//!
//! Single-number developer productivity metrics — lines of code, commit
//! count, story points — are trivially gamed and routinely mislead. A team
//! can be highly active while performing poorly, or perform well in the
//! short term while satisfaction craters, a leading indicator of the
//! attrition and quality collapse that shows up months later. A
//! single-dimension metric set is a known anti-pattern: the book insists on
//! covering multiple dimensions together, mixing subjective (survey) and
//! objective (instrumented) data sources.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::space_framework::{
//!     SpaceDimension, covers_all_dimensions, missing_dimensions,
//! };
//!
//! // A team that tracks only Activity is the book's classic anti-pattern.
//! let activity_only = [SpaceDimension::Activity];
//! assert!(!covers_all_dimensions(&activity_only));
//! assert_eq!(missing_dimensions(&activity_only).len(), 4);
//!
//! // A team that tracks all five is fully covered.
//! let all = [
//!     SpaceDimension::Satisfaction,
//!     SpaceDimension::Performance,
//!     SpaceDimension::Activity,
//!     SpaceDimension::Communication,
//!     SpaceDimension::Efficiency,
//! ];
//! assert!(covers_all_dimensions(&all));
//! assert!(missing_dimensions(&all).is_empty());
//! ```
//!
//! ## Pitfalls
//!
//! - **Adopting SPACE in name only** while remaining activity-dominated in
//!   practice defeats the framework's entire purpose.
//! - **Applying SPACE dimensions to individual scorecards** misapplies a
//!   framework validated for team and system-level insight, not individual
//!   performance.
//! - **Reviewing dimensions in isolation** rather than watching for
//!   cross-dimensional trade-offs misses the pattern SPACE is specifically
//!   designed to catch.
//! - **Treating a single satisfaction survey score as sufficient** without
//!   objective data loses the balance the framework calls for.
//!
//! ## Sources
//!
//! - Chapter 3.1, The SPACE framework.
//! - Forsgren, Storey, Maddila, Zimmermann, Houck, and Butler, "The SPACE of
//!   Developer Productivity," *ACM Queue* (2021).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/03-01-the-space-framework.md

/// One of the five SPACE dimensions.
///
/// Canonical order follows the acronym: Satisfaction, Performance, Activity,
/// Communication, Efficiency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpaceDimension {
    /// Satisfaction and well-being.
    Satisfaction,
    /// Performance (outcomes, not output).
    Performance,
    /// Activity (commits, pull requests, and similar counts) — the
    /// dimension most prone to misuse as a standalone proxy.
    Activity,
    /// Communication and collaboration.
    Communication,
    /// Efficiency and flow (absence of friction, sustained focus).
    Efficiency,
}

/// The five canonical SPACE dimensions, in acronym order.
const ALL_DIMENSIONS: [SpaceDimension; 5] = [
    SpaceDimension::Satisfaction,
    SpaceDimension::Performance,
    SpaceDimension::Activity,
    SpaceDimension::Communication,
    SpaceDimension::Efficiency,
];

/// Whether a measured set of dimensions covers all five SPACE dimensions.
///
/// The book's recommended minimum is at least three of five dimensions with
/// mixed subjective and objective sources; this function checks full
/// coverage (all five), which `missing_dimensions` can help diagnose
/// against that minimum.
///
/// # Arguments
///
/// * `measured` — the SPACE dimensions currently measured by a team.
///
/// # Returns
///
/// `true` iff every one of the five canonical dimensions appears in
/// `measured` (duplicates are ignored).
///
/// # Examples
///
/// ```rust
/// use software_engineering::space_framework::{SpaceDimension, covers_all_dimensions};
///
/// let activity_only = [SpaceDimension::Activity];
/// assert!(!covers_all_dimensions(&activity_only));
/// ```
#[must_use]
pub fn covers_all_dimensions(measured: &[SpaceDimension]) -> bool {
    ALL_DIMENSIONS.iter().all(|d| measured.contains(d))
}

/// The SPACE dimensions not present in a measured set, in canonical order.
///
/// # Arguments
///
/// * `measured` — the SPACE dimensions currently measured by a team.
///
/// # Returns
///
/// A vector of the missing dimensions, in the order Satisfaction,
/// Performance, Activity, Communication, Efficiency. Empty when `measured`
/// already covers all five.
///
/// # Examples
///
/// ```rust
/// use software_engineering::space_framework::{SpaceDimension, missing_dimensions};
///
/// // A single-dimension metric set is the book's classic anti-pattern.
/// let activity_only = [SpaceDimension::Activity];
/// let missing = missing_dimensions(&activity_only);
/// assert_eq!(missing.len(), 4);
/// assert!(missing.contains(&SpaceDimension::Satisfaction));
/// ```
#[must_use]
pub fn missing_dimensions(measured: &[SpaceDimension]) -> Vec<SpaceDimension> {
    ALL_DIMENSIONS
        .iter()
        .copied()
        .filter(|d| !measured.contains(d))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // "SPACE proposes measuring across five dimensions instead: Satisfaction
    // and well-being, Performance, Activity, Communication and
    // collaboration, and Efficiency and flow."
    #[test]
    fn all_five_dimensions_are_covered_when_all_are_measured() {
        let all = [
            SpaceDimension::Satisfaction,
            SpaceDimension::Performance,
            SpaceDimension::Activity,
            SpaceDimension::Communication,
            SpaceDimension::Efficiency,
        ];
        assert!(covers_all_dimensions(&all));
        assert!(missing_dimensions(&all).is_empty());
    }

    // "A metric set drawn entirely from one dimension or one data type is
    // not really using SPACE." — activity-only is the book's named
    // anti-pattern.
    #[test]
    fn activity_only_metric_set_is_not_full_coverage() {
        let activity_only = [SpaceDimension::Activity];
        assert!(!covers_all_dimensions(&activity_only));
        let missing = missing_dimensions(&activity_only);
        assert_eq!(missing.len(), 4);
        assert!(!missing.contains(&SpaceDimension::Activity));
        assert!(missing.contains(&SpaceDimension::Satisfaction));
        assert!(missing.contains(&SpaceDimension::Performance));
        assert!(missing.contains(&SpaceDimension::Communication));
        assert!(missing.contains(&SpaceDimension::Efficiency));
    }

    // "At least one metric from at least three dimensions ... is the
    // minimum for a balanced picture."
    #[test]
    fn three_dimensions_still_leaves_two_missing() {
        let three = [
            SpaceDimension::Satisfaction,
            SpaceDimension::Performance,
            SpaceDimension::Efficiency,
        ];
        assert!(!covers_all_dimensions(&three));
        assert_eq!(missing_dimensions(&three).len(), 2);
    }

    // Missing dimensions are reported in canonical acronym order.
    #[test]
    fn missing_dimensions_preserve_canonical_order() {
        let empty: [SpaceDimension; 0] = [];
        let missing = missing_dimensions(&empty);
        assert_eq!(
            missing,
            vec![
                SpaceDimension::Satisfaction,
                SpaceDimension::Performance,
                SpaceDimension::Activity,
                SpaceDimension::Communication,
                SpaceDimension::Efficiency,
            ]
        );
    }
}

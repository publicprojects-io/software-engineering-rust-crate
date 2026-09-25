//! # Escaped Defect Rate and Quality Escapes
//!
//! An **escaped defect** is one that reaches production rather than being
//! caught before release. The escaped defect rate compares how many defects
//! escaped against how many were found in total (pre- and post-release
//! combined), giving a direct read on how well internal quality practices
//! are catching problems before customers do. A raw count understates the
//! picture: weighting by severity, using a consistent, documented scale,
//! stops a spike in minor issues from visually swamping a smaller but far
//! more consequential rise in critical ones.
//!
//! ## Formula
//!
//! ```text
//! Escaped defect rate (%) = escaped defects / (escaped defects + caught defects) × 100
//! Severity-weighted score  = critical × 5 + major × 3 + minor × 1
//! ```
//!
//! ## Why it matters
//!
//! Classifying every escaped defect on a fixed severity scale, based on
//! actual customer or business impact, and tracking a severity-weighted
//! trend rather than just a raw count, is what keeps a handful of critical
//! escapes from being buried under a much larger count of cosmetic ones.
//! Standardizing classification criteria across teams matters just as much:
//! left to classify independently, teams drift toward different standards,
//! making cross-team comparison meaningless and creating an incentive to
//! classify generously downward to flatter a team's own numbers.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::escaped_defects::{
//!     escaped_defect_rate_percent, severity_weighted_escaped_defect_score,
//! };
//!
//! // 4 defects escaped to production out of 40 found in total.
//! let rate = escaped_defect_rate_percent(4.0, 36.0).unwrap();
//! assert_eq!(rate, 10.0);
//!
//! // 1 critical escape outweighs 4 minor ones under severity weighting.
//! let one_critical = severity_weighted_escaped_defect_score(1.0, 0.0, 0.0);
//! let four_minor = severity_weighted_escaped_defect_score(0.0, 0.0, 4.0);
//! assert!(one_critical > four_minor);
//! // But 10 minors already outweigh a single critical.
//! let ten_minor = severity_weighted_escaped_defect_score(0.0, 0.0, 10.0);
//! assert!(ten_minor > one_critical);
//! // And 3 criticals outweigh those same 10 minors.
//! let three_critical = severity_weighted_escaped_defect_score(3.0, 0.0, 0.0);
//! assert!(three_critical > ten_minor);
//! ```
//!
//! ## Pitfalls
//!
//! - **Tracking a raw escaped-defect count** instead of a severity-weighted
//!   trend — lets a spike in minor issues visually swamp a smaller, more
//!   consequential rise in critical ones.
//! - **Letting teams classify severity independently**, without a
//!   documented, audited scale — produces cross-team comparisons that are
//!   meaningless at best and gamed at worst.
//! - **Tracking count and severity without root cause** — misses the
//!   systemic pattern (a testing gap, a missed edge case, an
//!   environment difference) that would point at a specific, fixable
//!   process gap.
//! - **Framing defect classification as an individual-blame exercise** —
//!   creates a strong incentive to under-report or misclassify downward.
//!
//! ## Sources
//!
//! - Chapter 5.1, Escaped defect rate and quality escapes.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/05-01-escaped-defect-rate-and-quality-escapes.md

/// Weight applied to a critical-severity escaped defect in
/// [`severity_weighted_escaped_defect_score`].
///
/// A common, documented convention, not a universal constant — teams should
/// adapt the scale to their own context, per the chapter's "consistent,
/// documented scale" guidance.
pub const CRITICAL_WEIGHT: f64 = 5.0;

/// Weight applied to a major-severity escaped defect in
/// [`severity_weighted_escaped_defect_score`]. See [`CRITICAL_WEIGHT`].
pub const MAJOR_WEIGHT: f64 = 3.0;

/// Weight applied to a minor-severity escaped defect in
/// [`severity_weighted_escaped_defect_score`]. See [`CRITICAL_WEIGHT`].
pub const MINOR_WEIGHT: f64 = 1.0;

/// Escaped defect rate: the percentage of all found defects that escaped to
/// production rather than being caught first.
///
/// `escaped_defects / (escaped_defects + caught_defects) × 100`.
///
/// # Arguments
///
/// * `escaped_defects` — count of defects found in production.
/// * `caught_defects` — count of defects found before release.
///
/// # Returns
///
/// The escaped defect rate as a percentage, or `None` if both counts are
/// zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::escaped_defects::escaped_defect_rate_percent;
///
/// assert_eq!(escaped_defect_rate_percent(4.0, 36.0), Some(10.0));
/// assert_eq!(escaped_defect_rate_percent(0.0, 0.0), None);
/// ```
#[must_use]
pub fn escaped_defect_rate_percent(escaped_defects: f64, caught_defects: f64) -> Option<f64> {
    let total = escaped_defects + caught_defects;
    if total == 0.0 {
        return None;
    }
    Some((escaped_defects / total) * 100.0)
}

/// A severity-weighted escaped-defect score, so a spike in minor issues
/// cannot visually swamp a smaller rise in critical ones.
///
/// `critical × `[`CRITICAL_WEIGHT`]` + major × `[`MAJOR_WEIGHT`]` + minor ×
/// `[`MINOR_WEIGHT`].
///
/// # Arguments
///
/// * `critical` — count of critical-severity escaped defects.
/// * `major` — count of major-severity escaped defects.
/// * `minor` — count of minor-severity escaped defects.
///
/// # Returns
///
/// The severity-weighted score.
///
/// # Examples
///
/// ```rust
/// use software_engineering::escaped_defects::severity_weighted_escaped_defect_score;
///
/// // A single critical escape outweighs 4 minor ones.
/// let critical = severity_weighted_escaped_defect_score(1.0, 0.0, 0.0);
/// let minors = severity_weighted_escaped_defect_score(0.0, 0.0, 4.0);
/// assert!(critical > minors);
/// ```
#[must_use]
pub fn severity_weighted_escaped_defect_score(critical: f64, major: f64, minor: f64) -> f64 {
    critical * CRITICAL_WEIGHT + major * MAJOR_WEIGHT + minor * MINOR_WEIGHT
}

#[cfg(test)]
mod tests {
    use super::*;

    // "escaped defect rate" as a share of all defects found, pre- and
    // post-release combined.
    #[test]
    fn escaped_defect_rate_computes_percentage_of_total_found() {
        let rate = escaped_defect_rate_percent(4.0, 36.0).unwrap();
        assert!((rate - 10.0).abs() < 1e-9);
    }

    #[test]
    fn escaped_defect_rate_is_none_when_nothing_was_found() {
        assert_eq!(escaped_defect_rate_percent(0.0, 0.0), None);
    }

    // "Track a severity-weighted trend, not just a raw count, so that a
    // spike in minor issues does not visually swamp a smaller but far more
    // consequential increase in critical ones."
    #[test]
    fn a_handful_of_criticals_outweighs_many_minors() {
        let ten_minor = severity_weighted_escaped_defect_score(0.0, 0.0, 10.0);
        let three_critical = severity_weighted_escaped_defect_score(3.0, 0.0, 0.0);
        assert!(three_critical > ten_minor);
    }

    #[test]
    fn severity_weighted_score_is_zero_with_no_escapes() {
        let score = severity_weighted_escaped_defect_score(0.0, 0.0, 0.0);
        assert!((score - 0.0).abs() < 1e-9);
    }
}

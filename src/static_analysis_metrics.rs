//! # Static Analysis and Code Smell Metrics
//!
//! [Static analysis](https://en.wikipedia.org/wiki/Static_program_analysis)
//! tools scan source code without executing it, flagging patterns known
//! to correlate with defects, security vulnerabilities, or
//! maintainability problems, plus the broader category of code smells —
//! structural patterns that are not necessarily bugs but tend to make
//! code harder to understand, test, or safely change. The gap this
//! module addresses is between what a tool reports and what actually
//! matters: a raw finding count conflates trivial style preferences with
//! genuine, severe risk, and it can be driven down through suppression
//! as easily as through real fixes.
//!
//! ## Formula
//!
//! ```text
//! Findings per KLOC = findings / (lines_of_code / 1000)
//!
//! Severity-weighted score = Σ (count × severity_weight)
//!     Critical = 5, Major = 3, Minor = 1
//! ```
//!
//! ## Why it matters
//!
//! The value of static analysis comes not from the raw finding count but
//! from how well an organization triages severity: a small number of
//! critical findings deserves more attention than a large number of
//! trivial ones. Reporting only a total count invites exactly the wrong
//! incentive — suppressing findings (real or not) to make the number
//! smaller — while a severity-weighted score keeps a spike in trivial
//! findings from visually swamping a smaller but far more consequential
//! rise in critical ones.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::static_analysis_metrics::{
//!     FindingSeverity, findings_per_kloc, severity_weighted_finding_score,
//! };
//!
//! // 45 findings across 15,000 lines of code: 3 findings per KLOC.
//! let density = findings_per_kloc(45.0, 15_000.0).unwrap();
//! assert!((density - 3.0).abs() < 1e-9);
//!
//! // A single critical finding outweighs four minor ones — the
//! // severity-weighted score is what keeps that visible.
//! let critical = severity_weighted_finding_score(&[(FindingSeverity::Critical, 1.0)]);
//! let many_minor = severity_weighted_finding_score(&[(FindingSeverity::Minor, 4.0)]);
//! assert!(critical > many_minor);
//! ```
//!
//! ## Pitfalls
//!
//! - **Treating raw finding count as the metric** — conflates trivial
//!   and severe issues and is easily gamed through suppression.
//! - **Requiring the entire historical backlog resolved before any new
//!   work proceeds** — usually impractical and drives suppression rather
//!   than genuine fixes.
//! - **Ignoring false-positive rate** — an unmanaged noise level leads
//!   teams to tune out the tool's output entirely, including real
//!   findings.
//! - **Silent, undocumented suppression of legitimate findings** —
//!   erodes the tool's signal and leaves no audit trail.
//! - **Treating a finding as an automatic verdict with no human review**
//!   — misses context a tool cannot see.
//!
//! ## Sources
//!
//! - Chapter 4.4, Static analysis and code smell metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/04-04-static-analysis-and-code-smell-metrics.md

/// Severity classification for a static-analysis finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FindingSeverity {
    /// A finding with severe, direct risk (e.g. a likely defect or
    /// security exposure).
    Critical,
    /// A finding with meaningful but non-severe risk.
    Major,
    /// A trivial, mostly stylistic finding.
    Minor,
}

/// The fixed weight applied to a finding of a given severity in
/// [`severity_weighted_finding_score`]: Critical = 5.0, Major = 3.0,
/// Minor = 1.0 — the same weighting convention this crate uses for
/// escaped defects, so a spike in trivial findings can't visually swamp
/// a smaller but more consequential rise in critical ones.
///
/// # Arguments
///
/// * `severity` — the finding severity to weight.
///
/// # Returns
///
/// The fixed weight for that severity.
///
/// # Examples
///
/// ```rust
/// use software_engineering::static_analysis_metrics::{FindingSeverity, severity_weight};
///
/// assert!((severity_weight(FindingSeverity::Critical) - 5.0).abs() < 1e-9);
/// assert!((severity_weight(FindingSeverity::Major) - 3.0).abs() < 1e-9);
/// assert!((severity_weight(FindingSeverity::Minor) - 1.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn severity_weight(severity: FindingSeverity) -> f64 {
    match severity {
        FindingSeverity::Critical => 5.0,
        FindingSeverity::Major => 3.0,
        FindingSeverity::Minor => 1.0,
    }
}

/// Findings per thousand lines of code (KLOC) — a normalized density
/// that resists the "the codebase just got bigger" confound a raw
/// finding count has.
///
/// `findings / (lines_of_code / 1000)`.
///
/// # Arguments
///
/// * `findings` — total finding count.
/// * `lines_of_code` — size of the scanned codebase, in lines.
///
/// # Returns
///
/// The finding density per KLOC, or `None` if `lines_of_code` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::static_analysis_metrics::findings_per_kloc;
///
/// assert_eq!(findings_per_kloc(45.0, 15_000.0), Some(3.0));
/// assert_eq!(findings_per_kloc(45.0, 0.0), None);
/// ```
#[must_use]
pub fn findings_per_kloc(findings: f64, lines_of_code: f64) -> Option<f64> {
    if lines_of_code == 0.0 {
        return None;
    }
    Some(findings / (lines_of_code / 1000.0))
}

/// A severity-weighted static-analysis finding score, so a spike in
/// trivial findings cannot visually swamp a smaller rise in critical
/// ones.
///
/// Sum over `counts` of `count × severity_weight(severity)`.
///
/// # Arguments
///
/// * `counts` — pairs of (severity, count) for the findings being
///   scored.
///
/// # Returns
///
/// The severity-weighted score. An empty slice sums to `0.0`.
///
/// # Examples
///
/// ```rust
/// use software_engineering::static_analysis_metrics::{FindingSeverity, severity_weighted_finding_score};
///
/// // One critical finding (weight 5) outweighs four minor ones (weight 1 each = 4).
/// let critical = severity_weighted_finding_score(&[(FindingSeverity::Critical, 1.0)]);
/// let many_minor = severity_weighted_finding_score(&[(FindingSeverity::Minor, 4.0)]);
/// assert!(critical > many_minor);
/// assert_eq!(severity_weighted_finding_score(&[]), 0.0);
/// ```
#[must_use]
pub fn severity_weighted_finding_score(counts: &[(FindingSeverity, f64)]) -> f64 {
    counts
        .iter()
        .map(|&(severity, count)| count * severity_weight(severity))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_weight_matches_documented_values_for_all_severities() {
        assert!((severity_weight(FindingSeverity::Critical) - 5.0).abs() < 1e-9);
        assert!((severity_weight(FindingSeverity::Major) - 3.0).abs() < 1e-9);
        assert!((severity_weight(FindingSeverity::Minor) - 1.0).abs() < 1e-9);
    }

    // "Findings per KLOC" as a normalized density that resists a
    // growing-codebase confound.
    #[test]
    fn findings_per_kloc_divides_by_thousand_lines() {
        let density = findings_per_kloc(45.0, 15_000.0).unwrap();
        assert!((density - 3.0).abs() < 1e-9);
    }

    #[test]
    fn findings_per_kloc_is_none_for_zero_lines_of_code() {
        assert_eq!(findings_per_kloc(45.0, 0.0), None);
    }

    // "A small number of critical findings deserves more attention than
    // a large number of trivial ones."
    #[test]
    fn one_critical_finding_outweighs_several_minor_findings() {
        let critical = severity_weighted_finding_score(&[(FindingSeverity::Critical, 1.0)]);
        let many_minor = severity_weighted_finding_score(&[(FindingSeverity::Minor, 4.0)]);
        assert!(critical > many_minor);
    }

    #[test]
    fn empty_counts_sum_to_zero() {
        assert!((severity_weighted_finding_score(&[]) - 0.0).abs() < 1e-9);
    }
}

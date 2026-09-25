//! # Code Churn and Hotspot Analysis
//!
//! **Code churn** measures how frequently a file or module changes over
//! time — lines added, modified, and deleted across successive commits. On
//! its own, churn is a fairly weak signal: some files change often because
//! they are under active, healthy development, and some rarely change
//! because they are stable, not neglected. The diagnostic power comes from
//! combining churn with complexity: a file that is both frequently changed
//! and highly complex — a **hotspot** — is disproportionately likely to be
//! a source of defects and a drag on team velocity.
//!
//! ## Formula
//!
//! ```text
//! Code churn    = lines added + lines modified + lines deleted
//! Hotspot score = churn × complexity
//!
//! churn      = change volume for a file over a window (commonly 6-12 months)
//! complexity = a static-complexity measure for the same file (chapter 4.1)
//! ```
//!
//! ## Why it matters
//!
//! Hotspot analysis requires no manual survey: version control history
//! already contains everything needed to compute churn, and combined with
//! static analysis tooling, complexity, for every file automatically. Rank
//! files by the combination — commonly the product of churn and
//! complexity — rather than by either metric alone, since research
//! consistently associates that combination with elevated defect rates and
//! maintenance cost.
//!
//! ## Example
//!
//! The topic doc's government example: a licensing system's hotspot
//! analysis identified a cluster of files representing under 3% of the
//! total codebase that accounted for nearly 40% of all reported system
//! defects over the previous three years — a small, high-churn,
//! high-complexity cluster outranking the rest of the codebase combined.
//!
//! ```rust
//! use software_engineering::code_churn::{code_churn, hotspot_score};
//!
//! // A small cluster with heavy churn (many changed lines)...
//! let cluster_churn = code_churn(220.0, 140.0, 60.0);
//! assert_eq!(cluster_churn, 420.0);
//!
//! // ...and high complexity outranks a large, low-churn, low-complexity
//! // file, exactly the "churn combined with complexity" ranking method.
//! let cluster_score = hotspot_score(cluster_churn, 12.0);
//! let quiet_file_score = hotspot_score(code_churn(30.0, 10.0, 5.0), 3.0);
//! assert!(cluster_score > quiet_file_score);
//! ```
//!
//! ## Pitfalls
//!
//! - **Using churn alone without complexity** — a weak signal on its own
//!   that can flag healthy, actively developed code as a false positive.
//! - **Treating a hotspot ranking as an automatic action list** with no
//!   human judgement — misses whether the change is essential or accidental
//!   complexity.
//! - **Prioritizing refactoring by the loudest complaint** rather than the
//!   evidence, which frequently misdirects effort away from where the data
//!   shows the problem actually lives.
//! - **Never cross-referencing hotspots against incident or defect data**,
//!   missing the validation step that strengthens the case for acting.
//! - **Running the analysis once and never repeating it**, missing whether
//!   remediation is actually working over time.
//!
//! ## Sources
//!
//! - Chapter 4.3, Code churn and hotspot analysis.
//! - Tornhill, Adam, *Your Code as a Crime Scene*.
//! - Nagappan, Nachiappan, and Thomas Ball, "Use of Relative Code Churn
//!   Measures to Predict System Defect Density," *ICSE* (2005).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/04-03-code-churn-and-hotspot-analysis.md

/// Code churn: lines added + lines modified + lines deleted over a window.
///
/// Churn on its own is a weak signal — pair it with a complexity measure
/// via [`hotspot_score`] to identify genuine hotspots rather than merely
/// actively developed files.
///
/// # Arguments
///
/// * `lines_added` — lines added across the commits in the window.
/// * `lines_modified` — lines modified across the commits in the window.
/// * `lines_deleted` — lines deleted across the commits in the window.
///
/// # Returns
///
/// The total churn (sum of added, modified, and deleted lines).
///
/// # Examples
///
/// ```rust
/// use software_engineering::code_churn::code_churn;
///
/// // "lines added, modified, and deleted across successive commits."
/// assert_eq!(code_churn(220.0, 140.0, 60.0), 420.0);
/// ```
#[must_use]
pub fn code_churn(lines_added: f64, lines_modified: f64, lines_deleted: f64) -> f64 {
    lines_added + lines_modified + lines_deleted
}

/// Hotspot score: churn × complexity, the combined ranking signal.
///
/// Rank files by this combination, not by either churn or complexity
/// alone — the underlying research consistently associates the combination
/// with elevated defect rates and maintenance cost. A hotspot ranking is a
/// prioritization signal, not an automatic verdict; investigate top-ranked
/// files with human judgement before acting.
///
/// # Arguments
///
/// * `churn` — a file's code churn over the analysis window (see
///   [`code_churn`]).
/// * `complexity` — a complexity measure for the same file (chapter 4.1).
///
/// # Returns
///
/// The hotspot score (higher indicates a stronger hotspot candidate).
///
/// # Examples
///
/// ```rust
/// use software_engineering::code_churn::hotspot_score;
///
/// // "rank files by the combination, commonly the product of churn and
/// // complexity, rather than by either metric alone."
/// assert_eq!(hotspot_score(420.0, 12.0), 5_040.0);
/// ```
#[must_use]
pub fn hotspot_score(churn: f64, complexity: f64) -> f64 {
    churn * complexity
}

#[cfg(test)]
mod tests {
    use super::*;

    // "lines added, modified, and deleted across successive commits."
    #[test]
    fn code_churn_sums_added_modified_and_deleted_lines() {
        assert!((code_churn(220.0, 140.0, 60.0) - 420.0).abs() < 1e-9);
    }

    // "rank files by the combination, commonly the product of churn and
    // complexity, rather than by either metric alone."
    #[test]
    fn hotspot_score_is_the_product_of_churn_and_complexity() {
        assert!((hotspot_score(420.0, 12.0) - 5_040.0).abs() < 1e-9);
    }

    // "a small cluster of files, representing under 3% of the total
    // codebase, ... accounted for nearly 40% of all reported system
    // defects" — a small, high-churn, high-complexity cluster outranks a
    // larger, quieter file.
    #[test]
    fn small_high_churn_high_complexity_cluster_outranks_a_quiet_file() {
        let cluster_score = hotspot_score(code_churn(220.0, 140.0, 60.0), 12.0);
        let quiet_file_score = hotspot_score(code_churn(30.0, 10.0, 5.0), 3.0);
        assert!(cluster_score > quiet_file_score);
    }

    // "Churn alone is a weak signal" — churn with zero complexity produces
    // a zero hotspot score, illustrating that churn by itself does not
    // drive the ranking.
    #[test]
    fn churn_alone_without_complexity_scores_zero() {
        assert!((hotspot_score(code_churn(500.0, 200.0, 100.0), 0.0) - 0.0).abs() < 1e-9);
    }
}

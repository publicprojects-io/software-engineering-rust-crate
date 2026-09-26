//! # Performance Metrics and Outcome Proxies
//!
//! **Performance**, the P in SPACE (chapter 3.1), is the dimension most
//! often confused with activity: it asks whether work actually produced a
//! good outcome, not how much motion occurred. A team can be highly active
//! and low performing, shipping constant small changes that never move an
//! outcome, and the reverse is equally possible. Outcome is also rarely
//! attributable to a single person or team — it emerges from collaboration,
//! from decisions made months earlier, from market conditions no engineer
//! controls — so SPACE researchers were explicit that performance should be
//! measured at the system or team level using multiple, converging
//! signals, never reduced to a single number or attributed to an
//! individual.
//!
//! ## Formula
//!
//! ```text
//! Converging signal count      = count of independent signals indicating
//!                                 positive performance (change failure
//!                                 rate, defect-escape rate, adoption,
//!                                 qualitative peer assessment, ...)
//! Sufficient converging evidence = converging signal count >= 2
//!     (no single signal is reliable alone)
//! ```
//!
//! ## Why it matters
//!
//! No individual performance proxy is reliable enough to stand alone: a
//! single metric can look good while quality quietly degrades, or look bad
//! for reasons entirely outside a team's control. Requiring several
//! independent signals to agree before drawing a conclusion is what
//! resists both accidental misreading and deliberate gaming of any one
//! proxy — the same discipline chapter 5.3 applies to business-outcome
//! attribution, applied here to the SPACE performance dimension
//! specifically.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::performance_metrics::{
//!     converging_signal_count, has_sufficient_converging_evidence,
//! };
//!
//! // A single positive signal (adoption is up) is not enough on its own.
//! let adoption_only = [true, false, false];
//! assert_eq!(converging_signal_count(&adoption_only), 1);
//! assert!(!has_sufficient_converging_evidence(&adoption_only));
//!
//! // Change failure rate down, defect-escape rate down, and adoption up
//! // all agree: two or more converging signals are trustworthy together.
//! let converging = [true, true, false, true];
//! assert_eq!(converging_signal_count(&converging), 3);
//! assert!(has_sufficient_converging_evidence(&converging));
//! ```
//!
//! ## Pitfalls
//!
//! - **Reducing performance to a single number** — no individual proxy
//!   (velocity, adoption, a single quality metric) is reliable enough to
//!   stand alone.
//! - **Attributing an outcome to one individual** — software outcomes
//!   emerge from collaboration and prior work; individual attribution is
//!   usually false precision that discourages collaboration.
//! - **Treating quality as separate from performance** — a feature that
//!   ships on time but causes a wave of incidents did not perform well,
//!   even though an output-only view would count it as delivered.
//! - **Ranking teams competitively on performance data** — invites gaming
//!   and morale damage; the productive use is deciding where to invest or
//!   investigate, never a competitive ranking.
//! - **Forcing a direct-outcome metric onto platform or enabling teams** —
//!   their contribution is often several steps removed from any single
//!   customer-facing metric; measure their effect on the teams they
//!   enable instead.
//!
//! ## Sources
//!
//! - Chapter 3.3, Performance metrics and outcome proxies.
//! - Forsgren, Storey, Maddila, Zimmermann, Houck, and Butler, "The SPACE of
//!   Developer Productivity," *ACM Queue* (2021).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/03-03-performance-metrics-and-outcome-proxies.md

/// The number of independent signals, out of those checked, that indicate
/// positive performance.
///
/// Each `bool` in `signals` represents one independent signal already
/// evaluated by the caller (e.g. "did change failure rate improve",
/// "did defect-escape rate improve", "did adoption rise"), `true` if that
/// signal points to good performance. This function only counts how many
/// agree; it does not itself decide what counts as a signal.
///
/// # Arguments
///
/// * `signals` — one `bool` per independent performance signal checked,
///   `true` if that signal is positive.
///
/// # Returns
///
/// The count of `true` signals.
///
/// # Examples
///
/// ```rust
/// use software_engineering::performance_metrics::converging_signal_count;
///
/// let signals = [true, false, true, true];
/// assert_eq!(converging_signal_count(&signals), 3);
/// assert_eq!(converging_signal_count(&[]), 0);
/// ```
#[must_use]
pub fn converging_signal_count(signals: &[bool]) -> usize {
    signals.iter().filter(|signal| **signal).count()
}

/// Whether enough independent signals converge to trust a performance
/// conclusion, per the chapter's "no single one is reliable alone"
/// principle.
///
/// True iff [`converging_signal_count`] is at least 2 — a single positive
/// signal is never treated as sufficient evidence on its own.
///
/// # Arguments
///
/// * `signals` — one `bool` per independent performance signal checked,
///   `true` if that signal is positive.
///
/// # Returns
///
/// `true` if two or more signals are positive.
///
/// # Examples
///
/// ```rust
/// use software_engineering::performance_metrics::has_sufficient_converging_evidence;
///
/// assert!(!has_sufficient_converging_evidence(&[true]));
/// assert!(has_sufficient_converging_evidence(&[true, true]));
/// assert!(has_sufficient_converging_evidence(&[true, false, true]));
/// ```
#[must_use]
pub fn has_sufficient_converging_evidence(signals: &[bool]) -> bool {
    converging_signal_count(signals) >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Use multiple, converging signals, never a single performance
    // number. No individual proxy is reliable enough to stand alone."
    #[test]
    fn a_single_positive_signal_is_not_sufficient_evidence() {
        let adoption_only = [true, false, false];
        assert_eq!(converging_signal_count(&adoption_only), 1);
        assert!(!has_sufficient_converging_evidence(&adoption_only));
    }

    #[test]
    fn two_or_more_positive_signals_are_sufficient_evidence() {
        let converging = [true, true, false, true];
        assert_eq!(converging_signal_count(&converging), 3);
        assert!(has_sufficient_converging_evidence(&converging));
    }

    #[test]
    fn no_signals_checked_counts_as_zero_and_is_not_sufficient() {
        assert_eq!(converging_signal_count(&[]), 0);
        assert!(!has_sufficient_converging_evidence(&[]));
    }

    #[test]
    fn all_negative_signals_count_as_zero() {
        assert_eq!(converging_signal_count(&[false, false, false]), 0);
        assert!(!has_sufficient_converging_evidence(&[false, false, false]));
    }
}

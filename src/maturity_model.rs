//! # Maturity Model for Engineering Metrics Programs
//!
//! A metrics program's overall maturity is scored across five dimensions:
//! **governance**, **instrumentation**, **outcome balance**, **cultural
//! trust**, and **continuous improvement**, each independently on a 1–5
//! level scale (Level 1, Initiate, through Level 5, Orchestrate). The
//! chapter's central recommendation is to take the *minimum* across
//! dimensions as the honest overall score, resisting the temptation to
//! average them into a more flattering composite.
//!
//! ## Formula
//!
//! ```text
//! Honest overall score = minimum(governance, instrumentation, outcome balance,
//!                                 cultural trust, continuous improvement)
//! (Average is computed alongside it only to make the gap visible.)
//! ```
//!
//! ## Why it matters
//!
//! A programme with excellent instrumentation (Level 4) but weak cultural
//! trust (Level 1) is not, in any meaningful sense, a Level 2 or 3
//! programme; the weak dimension actively undermines the value of the
//! strong ones, since untrustworthy data corrupted by fear-driven gaming is
//! not rescued by having been collected with excellent instrumentation.
//! Reporting the minimum, even though it produces a less flattering overall
//! picture than an average would, is what keeps the assessment honest.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::maturity_model::{
//!     MaturityDimension, minimum_maturity_level, average_maturity_level,
//! };
//!
//! // Instrumentation through continuous improvement score well, but
//! // cultural trust lags badly.
//! let scored: [(MaturityDimension, u8); 5] = [
//!     (MaturityDimension::Governance, 4),
//!     (MaturityDimension::Instrumentation, 4),
//!     (MaturityDimension::OutcomeBalance, 4),
//!     (MaturityDimension::CulturalTrust, 1),
//!     (MaturityDimension::ContinuousImprovement, 4),
//! ];
//! let scores: Vec<u8> = scored.iter().map(|(_, level)| *level).collect();
//!
//! let honest_score = minimum_maturity_level(&scores).unwrap();
//! let flattering_average = average_maturity_level(&scores).unwrap();
//! assert_eq!(honest_score, 1);
//! assert!((flattering_average - 3.4).abs() < 1e-9);
//! assert!((flattering_average as f64) > (honest_score as f64));
//! ```
//!
//! ## Pitfalls
//!
//! - **Averaging the five dimension scores** into a single, more flattering
//!   composite, instead of reporting the minimum — hides exactly the weak
//!   dimension that undermines the rest.
//! - **Assessing only aspirationally**, based on stated policy rather than
//!   concrete evidence for each dimension.
//! - **Reassessing only after a crisis** forces the question reactively,
//!   rather than on a fixed, regular cadence.
//! - **Treating a low score as a verdict to feel bad about**, rather than
//!   the diagnostic starting point for a targeted investment plan.
//!
//! ## Sources
//!
//! - Chapter 8.4, Maturity model for engineering metrics programs.
//! - *Capability Maturity Model Integration (CMMI)*, Software Engineering
//!   Institute (structural inspiration).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/08-04-maturity-model-for-engineering-metrics-programs.md

/// One of the five maturity-model dimensions, in the chapter's own order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaturityDimension {
    /// Whether every consequential metric has a named owner and a
    /// documented charter.
    Governance,
    /// Whether metrics come from automated sources rather than self-report
    /// wherever possible.
    Instrumentation,
    /// The actual ratio of outcome-weighted to output-weighted metrics on
    /// primary dashboards.
    OutcomeBalance,
    /// Whether rollout history has ever included a mishandled, punitive use
    /// of a metric, and how it was addressed.
    CulturalTrust,
    /// Whether the organization has a documented history of retiring
    /// metrics that stopped earning their keep.
    ContinuousImprovement,
}

/// The minimum score across a set of per-dimension maturity levels — the
/// chapter's recommended honest overall score.
///
/// Each score is expected to be in `1..=5`, but this function itself simply
/// takes the minimum of whatever values are given; validating the range is
/// the caller's responsibility.
///
/// # Arguments
///
/// * `scores` — one maturity level per dimension assessed.
///
/// # Returns
///
/// The minimum level, or `None` if `scores` is empty.
///
/// # Examples
///
/// ```rust
/// use software_engineering::maturity_model::minimum_maturity_level;
///
/// assert_eq!(minimum_maturity_level(&[4, 4, 4, 1, 4]), Some(1));
/// assert_eq!(minimum_maturity_level(&[]), None);
/// ```
#[must_use]
pub fn minimum_maturity_level(scores: &[u8]) -> Option<u8> {
    scores.iter().copied().min()
}

/// The arithmetic mean across a set of per-dimension maturity levels — the
/// more flattering composite the chapter explicitly warns against using as
/// the *overall* score. Kept here so callers can compute it alongside
/// [`minimum_maturity_level`] and see the gap between the two.
///
/// # Arguments
///
/// * `scores` — one maturity level per dimension assessed.
///
/// # Returns
///
/// The mean level as an `f64`, or `None` if `scores` is empty.
///
/// # Examples
///
/// ```rust
/// use software_engineering::maturity_model::average_maturity_level;
///
/// let average = average_maturity_level(&[4, 4, 4, 1, 4]).unwrap();
/// assert!((average - 3.4).abs() < 1e-9);
/// assert_eq!(average_maturity_level(&[]), None);
/// ```
#[must_use]
pub fn average_maturity_level(scores: &[u8]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    let sum: u32 = scores.iter().map(|&level| u32::from(level)).sum();
    // Maturity levels are always small integers (1..=5), so this cast is
    // always exact.
    #[allow(clippy::cast_precision_loss)]
    let count = scores.len() as f64;
    Some(f64::from(sum) / count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_maturity_level_is_none_for_empty_scores() {
        assert_eq!(minimum_maturity_level(&[]), None);
    }

    #[test]
    fn average_maturity_level_is_none_for_empty_scores() {
        assert_eq!(average_maturity_level(&[]), None);
    }

    // "Take the minimum across dimensions as your honest overall score...
    // A programme with excellent instrumentation (Level 4) but weak
    // cultural trust (Level 1) is not, in any meaningful sense, a Level 2
    // or 3 programme."
    #[test]
    fn minimum_reveals_the_weak_dimension_the_average_hides() {
        let scores = [4, 4, 4, 1, 4];
        let honest_score = minimum_maturity_level(&scores).unwrap();
        let flattering_average = average_maturity_level(&scores).unwrap();
        assert_eq!(honest_score, 1);
        assert!((flattering_average - 3.4).abs() < 1e-9);
        assert!(f64::from(honest_score) < flattering_average);
    }

    #[test]
    fn a_uniformly_scored_programme_has_matching_minimum_and_average() {
        let scores = [3, 3, 3, 3, 3];
        let minimum = minimum_maturity_level(&scores).unwrap();
        let average = average_maturity_level(&scores).unwrap();
        assert_eq!(minimum, 3);
        assert!((average - 3.0).abs() < 1e-9);
    }
}

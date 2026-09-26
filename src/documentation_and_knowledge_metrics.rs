//! # Documentation and Knowledge Metrics
//!
//! This module measures whether the knowledge needed to safely maintain
//! a codebase is actually documented and findable, not just whether
//! documentation technically exists somewhere: does a new engineer, or
//! an existing one working on unfamiliar code, have what they need to
//! make a safe change, or does that knowledge live only in the heads of
//! a shrinking number of tenured people. A system maintained for years
//! by the same two engineers can function perfectly well with almost no
//! written documentation, right up until both of those engineers leave
//! within the same year — at which point the knowledge is discovered to
//! have never been captured anywhere durable. That risk has a standard
//! name in the industry: the **bus factor** (or truck factor).
//!
//! ## Formula
//!
//! ```text
//! Bus factor = the minimum number of people whose combined knowledge
//!              share meets or exceeds a critical threshold (commonly 50%)
//!
//! At risk  when bus_factor <= minimum_safe_bus_factor
//! ```
//!
//! ## Why it matters
//!
//! Documentation existence is not the same as documentation usefulness —
//! counting wiki pages or READMEs tells you almost nothing about whether
//! knowledge is actually accessible when needed. Bus factor measures the
//! underlying risk directly: how concentrated is the knowledge needed to
//! safely change a system. A low bus factor can hide behind apparent
//! stability — a system that has not changed in years is not necessarily
//! low-risk, it may simply not have needed its sole expert yet — and
//! discovering the gap only during an emergency staff transition is
//! exactly the expensive, avoidable failure mode this module exists to
//! surface in advance.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::documentation_and_knowledge_metrics::{
//!     bus_factor, is_bus_factor_at_risk,
//! };
//!
//! // One person holds 60% of the knowledge share for a system: a single
//! // departure alone crosses the 50% critical threshold.
//! let concentrated = bus_factor(&[60.0, 25.0, 15.0], 50.0).unwrap();
//! assert_eq!(concentrated, 1);
//! assert!(is_bus_factor_at_risk(concentrated, 2));
//!
//! // Five people each hold an even 20% share: it takes three departures
//! // to cross the same threshold.
//! let spread_out = bus_factor(&[20.0, 20.0, 20.0, 20.0, 20.0], 50.0).unwrap();
//! assert_eq!(spread_out, 3);
//! assert!(!is_bus_factor_at_risk(spread_out, 2));
//! ```
//!
//! ## Pitfalls
//!
//! - **Counting documentation existence rather than usefulness** — tells
//!   you almost nothing about whether knowledge is actually accessible
//!   when needed.
//! - **Never checking documentation staleness relative to how much the
//!   system has changed** — risks actively misleading, out-of-date
//!   content.
//! - **Mistaking apparent stability for low risk** — a system that has
//!   not changed in years can mask a severe, undocumented bus-factor
//!   problem behind a system that simply has not yet needed its sole
//!   expert.
//! - **Discovering critical undocumented knowledge only during an
//!   emergency staff transition** — the expensive, avoidable failure mode
//!   this chapter is built to prevent.
//!
//! ## Sources
//!
//! - Chapter 4.6, Documentation and knowledge metrics.
//! - The "bus factor" (or "truck factor") is a widely used, informally
//!   named industry concept for knowledge-concentration risk.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/04-06-documentation-and-knowledge-metrics.md

/// The bus factor: the minimum number of people whose combined knowledge
/// share meets or exceeds `critical_threshold_percent` (commonly `50.0`).
///
/// Sorts a copy of `knowledge_shares_percent` in descending order and
/// accumulates from the largest share down, counting how many people it
/// takes to reach the threshold. A low result means knowledge is
/// dangerously concentrated in a few people; a high result means it is
/// spread widely.
///
/// # Arguments
///
/// * `knowledge_shares_percent` — each person's percentage share of
///   understanding or ownership of the system. Values are assumed to be
///   finite, non-NaN percentages (they need not sum to exactly `100.0`).
/// * `critical_threshold_percent` — the combined share (e.g. `50.0`)
///   whose loss is considered critical.
///
/// # Returns
///
/// The number of people whose combined share reaches the threshold, or
/// `None` if `knowledge_shares_percent` is empty, or if the shares never
/// reach `critical_threshold_percent` even after summing all of them.
///
/// # Examples
///
/// ```rust
/// use software_engineering::documentation_and_knowledge_metrics::bus_factor;
///
/// // One person alone holds 60%, past the 50% threshold.
/// assert_eq!(bus_factor(&[60.0, 25.0, 15.0], 50.0), Some(1));
///
/// // Five even 20% shares need three of them to cross 50%.
/// assert_eq!(bus_factor(&[20.0, 20.0, 20.0, 20.0, 20.0], 50.0), Some(3));
///
/// assert_eq!(bus_factor(&[], 50.0), None);
/// // Shares that never reach the threshold even combined.
/// assert_eq!(bus_factor(&[10.0, 10.0], 50.0), None);
/// ```
#[must_use]
pub fn bus_factor(knowledge_shares_percent: &[f64], critical_threshold_percent: f64) -> Option<usize> {
    if knowledge_shares_percent.is_empty() {
        return None;
    }
    let mut shares: Vec<f64> = knowledge_shares_percent.to_vec();
    // total_cmp gives a full ordering even for NaN, so this never panics.
    shares.sort_by(|a, b| b.total_cmp(a));

    let mut running_total = 0.0;
    for (index, share) in shares.iter().enumerate() {
        running_total += share;
        if running_total >= critical_threshold_percent {
            return Some(index + 1);
        }
    }
    None
}

/// Whether a bus factor is at or below a defined minimum-safe threshold
/// (e.g. a bus factor of 1 or 2 is commonly considered dangerously low).
///
/// True iff `bus_factor <= minimum_safe_bus_factor`.
///
/// # Arguments
///
/// * `bus_factor` — a bus factor computed by [`bus_factor`].
/// * `minimum_safe_bus_factor` — the smallest bus factor considered
///   acceptable.
///
/// # Returns
///
/// `true` if the bus factor is at or below the minimum-safe threshold.
///
/// # Examples
///
/// ```rust
/// use software_engineering::documentation_and_knowledge_metrics::is_bus_factor_at_risk;
///
/// assert!(is_bus_factor_at_risk(1, 2));
/// assert!(!is_bus_factor_at_risk(3, 2));
/// ```
#[must_use]
pub fn is_bus_factor_at_risk(bus_factor: usize, minimum_safe_bus_factor: usize) -> bool {
    bus_factor <= minimum_safe_bus_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    // "does that knowledge live only in the heads of a shrinking number
    // of tenured people" — one person holding a majority share is the
    // clearest, most concentrated case.
    #[test]
    fn one_concentrated_share_gives_a_bus_factor_of_one() {
        assert_eq!(bus_factor(&[60.0, 25.0, 15.0], 50.0), Some(1));
    }

    #[test]
    fn evenly_spread_shares_need_more_people_to_reach_the_threshold() {
        assert_eq!(bus_factor(&[20.0, 20.0, 20.0, 20.0, 20.0], 50.0), Some(3));
    }

    #[test]
    fn bus_factor_is_none_for_empty_shares() {
        assert_eq!(bus_factor(&[], 50.0), None);
    }

    #[test]
    fn bus_factor_is_none_when_threshold_is_never_reached() {
        assert_eq!(bus_factor(&[10.0, 10.0], 50.0), None);
    }

    // "a system maintained for years by the same two engineers can
    // function perfectly well... right up until both of those engineers
    // leave within the same year" — a bus factor of 1 or 2 is exactly
    // the dangerous case this chapter warns about.
    #[test]
    fn a_low_bus_factor_is_flagged_as_at_risk() {
        assert!(is_bus_factor_at_risk(1, 2));
    }

    #[test]
    fn a_high_bus_factor_is_not_flagged_as_at_risk() {
        assert!(!is_bus_factor_at_risk(3, 2));
    }
}

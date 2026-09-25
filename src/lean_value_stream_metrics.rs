//! # Lean Value Stream Metrics
//!
//! Every flow metric in this crate descends from the five baseline
//! measurements of classical Lean value stream mapping, developed at Toyota
//! and generalized across manufacturing, operations, and service delivery
//! long before software adopted them: lead time, process time, cycle time,
//! percent complete and accurate (%C/A), and takt time. This module
//! implements %C/A, rolled throughput yield, and takt time — the three that
//! do not already have a direct home elsewhere in this crate.
//!
//! ## Formula
//!
//! ```text
//! %C/A = usable units without rework / total units × 100%
//! Rolled throughput yield = %C/A(stage 1) × %C/A(stage 2) × ... × %C/A(stage N)
//! Takt time = available working time / customer demand over that period
//! ```
//!
//! ## Why it matters
//!
//! %C/A measures something the flow metrics do not: how much of what a
//! stage produces is actually usable by the next stage without being sent
//! back. Rolled up across a multi-stage value stream (rolled throughput
//! yield), it reveals how rework compounds invisibly across handoffs: three
//! stages each individually running at 90% complete-and-accurate compound to
//! roughly 73% overall — a number that looks nothing like any single
//! stage's own report and is usually the more honest one. Takt time
//! reframes capacity planning around real customer demand rather than
//! existing pace.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::lean_value_stream_metrics::{
//!     percent_complete_and_accurate, rolled_throughput_yield, takt_time,
//! };
//!
//! // A stage that produces 90 usable units out of 100: 90% C/A.
//! let stage_pca = percent_complete_and_accurate(90.0, 100.0).unwrap();
//! assert!((stage_pca - 90.0).abs() < 1e-9);
//!
//! // Three stages each at 90% C/A compound to about 73% rolled throughput yield.
//! let rty = rolled_throughput_yield(&[0.90, 0.90, 0.90]);
//! assert!((rty - 0.729).abs() < 1e-9);
//!
//! // 400 minutes of available working time against demand for 20 units: 20 min/unit.
//! let takt = takt_time(400.0, 20.0).unwrap();
//! assert_eq!(takt, 20.0);
//! ```
//!
//! ## Pitfalls
//!
//! - **Measuring %C/A only at final delivery**, the chapter's central
//!   gaming vector: a team can report a high final-stage %C/A while earlier
//!   stages quietly produce rework fixed before anyone measures it. Roll
//!   %C/A up multiplicatively across every stage instead.
//! - **Setting takt time from current capacity instead of real customer
//!   demand** defeats the purpose of the metric, which is to reveal a gap
//!   between demand and capacity.
//! - **Reporting %C/A without pairing it against flow velocity** allows a
//!   rising throughput number to hide a falling rework rate.
//!
//! ## Sources
//!
//! - Rother, Mike, and John Shook. *Learning to See: Value Stream Mapping to
//!   Create Value and Eliminate Muda*. Lean Enterprise Institute, 1999.
//! - Ohno, Taiichi. *Toyota Production System: Beyond Large-Scale
//!   Production*. Productivity Press, 1988.
//!
//! Topic doc: 02-08-lean-value-stream-metrics.md

/// Percent complete and accurate (%C/A): the share of a stage's output that
/// a downstream team can use without rework.
///
/// Measure %C/A at each stage individually so it can be rolled up
/// multiplicatively into [`rolled_throughput_yield`] — measuring it only at
/// final delivery hides rework introduced and caught earlier in the stream.
///
/// # Arguments
///
/// * `usable_without_rework` — units the downstream stage can process
///   without sending them back.
/// * `total_units` — total units the stage produced.
///
/// # Returns
///
/// `Some(percentage)` (e.g. `90.0` for 90%), or `None` when `total_units`
/// is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::lean_value_stream_metrics::percent_complete_and_accurate;
///
/// let pca = percent_complete_and_accurate(90.0, 100.0).unwrap();
/// assert!((pca - 90.0).abs() < 1e-9);
/// assert_eq!(percent_complete_and_accurate(90.0, 0.0), None);
/// ```
#[must_use]
pub fn percent_complete_and_accurate(usable_without_rework: f64, total_units: f64) -> Option<f64> {
    if total_units == 0.0 {
        None
    } else {
        Some(usable_without_rework / total_units * 100.0)
    }
}

/// Rolled throughput yield: the product of every stage's %C/A fraction
/// across a multi-stage value stream.
///
/// Three stages each individually running at 90% complete-and-accurate
/// compound to roughly 73% overall, a number that looks nothing like any
/// single stage's own report and is usually the more honest one.
///
/// # Arguments
///
/// * `stage_pca_fractions` — each stage's %C/A expressed as a fraction
///   (e.g. `0.9` for 90%), in stage order.
///
/// # Returns
///
/// The product of all fractions. An empty slice returns `1.0` (the identity
/// for multiplication — no stages, no compounding loss).
///
/// # Examples
///
/// ```rust
/// use software_engineering::lean_value_stream_metrics::rolled_throughput_yield;
///
/// let rty = rolled_throughput_yield(&[0.90, 0.90, 0.90]);
/// assert!((rty - 0.729).abs() < 1e-9);
///
/// assert_eq!(rolled_throughput_yield(&[]), 1.0);
/// ```
#[must_use]
pub fn rolled_throughput_yield(stage_pca_fractions: &[f64]) -> f64 {
    stage_pca_fractions.iter().product()
}

/// Takt time: the maximum acceptable time to complete a unit to cleanly
/// match customer demand.
///
/// Calculate takt time from real customer demand data, deliberately
/// independent of how fast the team happens to be able to work today. A
/// cycle time exceeding takt time is concrete, quantified evidence of a
/// capacity shortfall.
///
/// # Arguments
///
/// * `available_working_time` — total working time available in the period
///   (any consistent time unit).
/// * `customer_demand` — number of units demanded over that same period.
///
/// # Returns
///
/// `Some(takt time)` per unit, or `None` when `customer_demand` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::lean_value_stream_metrics::takt_time;
///
/// // 400 minutes of available time to meet demand for 20 units: 20 min/unit.
/// assert_eq!(takt_time(400.0, 20.0), Some(20.0));
/// assert_eq!(takt_time(400.0, 0.0), None);
/// ```
#[must_use]
pub fn takt_time(available_working_time: f64, customer_demand: f64) -> Option<f64> {
    if customer_demand == 0.0 {
        None
    } else {
        Some(available_working_time / customer_demand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Worked example: "Three stages each individually running at 90%
    // complete and accurate compound to roughly 73% overall" (chapter 2.8).
    #[test]
    fn three_stages_at_90_percent_compound_to_about_73_percent() {
        let rty = rolled_throughput_yield(&[0.90, 0.90, 0.90]);
        assert!((rty - 0.729).abs() < 1e-9);
    }

    // Worked example: a four-stage pipeline where stages that individually
    // look reasonable (95%, 90%, 85%, 83%) compound to a rolled throughput
    // yield well below any single stage.
    #[test]
    fn rolled_throughput_yield_of_61_percent_is_below_any_single_stage() {
        // Four stages that individually look reasonable but compound low.
        let rty = rolled_throughput_yield(&[0.95, 0.90, 0.85, 0.83]);
        assert!(rty < 0.90);
        assert!((rty - 0.603_202_5).abs() < 1e-9);
    }

    #[test]
    fn empty_rolled_throughput_yield_is_identity() {
        assert!((rolled_throughput_yield(&[]) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn percent_complete_and_accurate_divides_correctly() {
        let pca = percent_complete_and_accurate(90.0, 100.0).unwrap();
        assert!((pca - 90.0).abs() < 1e-9);
        assert_eq!(percent_complete_and_accurate(90.0, 0.0), None);
    }

    #[test]
    fn takt_time_divides_available_time_by_demand() {
        assert_eq!(takt_time(400.0, 20.0), Some(20.0));
        assert_eq!(takt_time(400.0, 0.0), None);
    }
}

//! # Flow Framework
//!
//! Mik Kersten's Flow Framework (from *Project to Product*) treats software
//! delivery as a value stream and defines five core measurements: flow
//! velocity, flow distribution, flow time, flow load, and flow efficiency.
//! This module implements the arithmetic behind flow time, flow load, and
//! the Little's law relationship that binds flow load to flow time.
//!
//! ## Formula
//!
//! ```text
//! Flow time  = t(delivery) − t(entry)
//! Flow load  = count of items currently active or waiting in the value stream
//! Little's law: flow load (WIP) = arrival rate × flow time (cycle time)
//! Flow efficiency = active time / total elapsed time × 100%
//!
//! t(entry)     = when a flow item enters the value stream
//! t(delivery)  = when a flow item is delivered
//! arrival rate = new items entering the value stream per unit time
//! ```
//!
//! ## Why it matters
//!
//! Flow load does not just correlate with flow time — via Little's law it
//! mathematically dictates it. If flow load keeps rising while arrival rate
//! stays flat, flow time is *guaranteed* to rise too. This turns "we're too
//! overloaded, things are taking too long" from a qualitative complaint into
//! a provable, quantitative argument a business leader cannot easily
//! dismiss. Flow efficiency captures a related, and usually surprising,
//! fact: most software delivery pipelines run between 10% and 25% flow
//! efficiency, meaning the dominant cost is wait time, not active effort.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::flow_framework::{
//!     flow_time, littles_law_wip, littles_law_flow_time, flow_efficiency_percent,
//! };
//!
//! // A flow item enters day 0 and is delivered day 12.
//! let t = flow_time(0.0, 12.0);
//! assert_eq!(t, 12.0);
//!
//! // Little's law: WIP = arrival rate (items/day) × flow time (days).
//! let wip = littles_law_wip(2.0, t);
//! assert_eq!(wip, 24.0);
//!
//! // Solve the other direction: given WIP and arrival rate, find flow time.
//! let recovered = littles_law_flow_time(wip, 2.0).unwrap();
//! assert!((recovered - t).abs() < 1e-9);
//!
//! // Ten active hours out of ninety total elapsed hours: 10% flow efficiency.
//! let efficiency = flow_efficiency_percent(10.0, 100.0).unwrap();
//! assert!((efficiency - 10.0).abs() < 1e-9);
//! ```
//!
//! ## Pitfalls
//!
//! - **Quietly narrowing the flow-time starting point** (e.g. from genuine
//!   business-need identification to engineering pickup) shrinks flow time
//!   without improving genuine responsiveness — document the entry point
//!   explicitly and audit it periodically.
//! - **Measuring flow load only periodically** forfeits its value as a
//!   leading indicator; track it continuously.
//! - **Applying a WIP limit as an individual quota** rather than a system
//!   constraint misapplies the technique and risks individual gaming.
//! - **Treating a low flow-efficiency number as a sign of a bad team**: 10%
//!   to 25% is typical of most delivery pipelines, and is a starting point
//!   for investigation, not a verdict.
//!
//! ## Sources
//!
//! - Kersten, Mik. *Project to Product: How to Survive and Thrive in the Age
//!   of Digital Disruption with the Flow Framework*. IT Revolution Press,
//!   2018.
//! - Little, John D. C. "A Proof for the Queuing Formula: L = λW." *Operations
//!   Research*, 1961.
//!
//! Topic doc: 02-01-the-flow-framework.md, 02-03-flow-velocity-and-flow-distribution.md,
//! 02-04-flow-time-and-flow-load.md, 02-05-flow-efficiency-and-work-in-process.md

/// Flow velocity: count of flow items completed per unit time.
///
/// Flow velocity is the Flow Framework's throughput measure (chapter 2.3).
/// It should always be reported alongside flow distribution — a rising item
/// count can hide a shift toward rework or item-splitting gaming, so
/// velocity alone is an incomplete picture.
///
/// # Arguments
///
/// * `items_completed` — count of flow items completed in the period.
/// * `period` — length of the observation period (any consistent time unit).
///
/// # Returns
///
/// `Some(items per unit period)`, or `None` when `period` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::flow_framework::flow_velocity;
///
/// // 40 items completed in a 4-week period = 10 items/week.
/// assert_eq!(flow_velocity(40.0, 4.0), Some(10.0));
/// assert_eq!(flow_velocity(40.0, 0.0), None);
/// ```
pub fn flow_velocity(items_completed: f64, period: f64) -> Option<f64> {
    if period == 0.0 {
        None
    } else {
        Some(items_completed / period)
    }
}

/// Flow distribution: the percentage share of one flow item type among all
/// completed items.
///
/// Flow distribution (chapter 2.3) answers "what kind of work was it" —
/// features, defects, risk, or debt. Report it alongside flow velocity,
/// never alone, so a rising item count that is quietly dominated by defect
/// rework is visible rather than mistaken for accelerating feature delivery.
///
/// # Arguments
///
/// * `items_of_type` — count of completed items of one flow item type.
/// * `total_completed_items` — count of all completed items in the period.
///
/// # Returns
///
/// `Some(percentage)` (e.g. `45.0` for 45%), or `None` when
/// `total_completed_items` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::flow_framework::flow_distribution_percent;
///
/// // The vendor's "features" share fell from 70% to 45% while velocity rose.
/// assert_eq!(flow_distribution_percent(45.0, 100.0), Some(45.0));
/// assert_eq!(flow_distribution_percent(1.0, 0.0), None);
/// ```
pub fn flow_distribution_percent(items_of_type: f64, total_completed_items: f64) -> Option<f64> {
    if total_completed_items == 0.0 {
        None
    } else {
        Some(items_of_type / total_completed_items * 100.0)
    }
}

/// Flow time: total elapsed time from a flow item entering the value stream
/// to its delivery.
///
/// Flow time (chapter 2.4) spans the *whole* value stream, from a business
/// need being identified to a customer receiving value — broader than
/// [`crate::cycle_time`], which covers only the engineering stages. It is
/// conceptually just elapsed time, `delivery − entry`, but the definition of
/// `entry` must be fixed and documented: quietly narrowing it is this
/// metric's central gaming risk.
///
/// # Arguments
///
/// * `entry` — the timestamp (any consistent unit) the item entered the
///   value stream.
/// * `delivery` — the timestamp the item was delivered.
///
/// # Returns
///
/// The elapsed flow time, `delivery − entry`.
///
/// # Examples
///
/// ```rust
/// use software_engineering::flow_framework::flow_time;
///
/// // Entered on day 3, delivered on day 15: 12 days of flow time.
/// assert_eq!(flow_time(3.0, 15.0), 12.0);
/// ```
pub fn flow_time(entry: f64, delivery: f64) -> f64 {
    delivery - entry
}

/// Flow load: total count of flow items currently active or waiting in the
/// value stream.
///
/// Flow load (chapter 2.4) is a count, not a computed ratio — it is what
/// [`crate::code_churn`]-style metrics are to churn, a raw tally that other
/// formulas (Little's law) then relate to other quantities. This helper
/// simply sums active and waiting items so the intent is explicit at the
/// call site.
///
/// # Arguments
///
/// * `active` — count of items currently being actively worked on.
/// * `waiting` — count of items currently waiting in a queue.
///
/// # Returns
///
/// The total flow load, `active + waiting`.
///
/// # Examples
///
/// ```rust
/// use software_engineering::flow_framework::flow_load_from_items;
///
/// // 5 items actively being worked, 17 waiting: flow load of 22.
/// assert_eq!(flow_load_from_items(5, 17), 22);
/// ```
pub fn flow_load_from_items(active: u32, waiting: u32) -> u32 {
    active + waiting
}

/// Little's law, solved for flow load (work in process): `WIP = arrival rate
/// × flow time`.
///
/// This is a proof from queueing theory (chapter 2.4, chapter 2.7), not a
/// heuristic: for any stable value stream, flow load equals arrival rate
/// multiplied by flow time. It is the single most persuasive tool in this
/// book for arguing that overloading a value stream provably slows every
/// item already in it.
///
/// # Arguments
///
/// * `arrival_rate` — new items entering the value stream per unit time.
/// * `flow_time` — average time an item spends in the value stream, in the
///   same time unit as `arrival_rate`'s denominator.
///
/// # Returns
///
/// The implied flow load (work in process).
///
/// # Examples
///
/// ```rust
/// use software_engineering::flow_framework::littles_law_wip;
///
/// // Arrival rate of 3 items/day, average flow time of 8 days: WIP = 24.
/// assert_eq!(littles_law_wip(3.0, 8.0), 24.0);
/// ```
pub fn littles_law_wip(arrival_rate: f64, flow_time: f64) -> f64 {
    arrival_rate * flow_time
}

/// Little's law, solved for flow time: `flow time = WIP / arrival rate`.
///
/// The inverse of [`littles_law_wip`], useful for checking whether measured
/// flow load, arrival rate, and flow time are internally consistent, or for
/// predicting flow time from a target WIP and known arrival rate.
///
/// # Arguments
///
/// * `wip` — flow load (work in process).
/// * `arrival_rate` — new items entering the value stream per unit time.
///
/// # Returns
///
/// `Some(flow time)`, or `None` when `arrival_rate` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::flow_framework::littles_law_flow_time;
///
/// // WIP of 24 items at an arrival rate of 3 items/day implies 8 days flow time.
/// assert_eq!(littles_law_flow_time(24.0, 3.0), Some(8.0));
/// assert_eq!(littles_law_flow_time(24.0, 0.0), None);
/// ```
pub fn littles_law_flow_time(wip: f64, arrival_rate: f64) -> Option<f64> {
    if arrival_rate == 0.0 {
        None
    } else {
        Some(wip / arrival_rate)
    }
}

/// Flow efficiency: the percentage of total elapsed time that was active
/// work.
///
/// Most software delivery pipelines, measured honestly, land between 10%
/// and 25% flow efficiency (chapter 2.5) — wait time, not active effort,
/// dominates. A low number is typical, not a sign of a broken team; it
/// redirects attention from "work harder" toward "reduce queueing."
///
/// # Arguments
///
/// * `active_time` — time actively spent coding, reviewing, or testing.
/// * `total_elapsed_time` — total time from start to finish, including wait
///   time, in the same unit as `active_time`.
///
/// # Returns
///
/// `Some(percentage)` (e.g. `10.0` for 10%), or `None` when
/// `total_elapsed_time` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::flow_framework::flow_efficiency_percent;
///
/// // 10 active hours out of 100 total elapsed hours: 10% flow efficiency.
/// assert_eq!(flow_efficiency_percent(10.0, 100.0), Some(10.0));
/// assert_eq!(flow_efficiency_percent(10.0, 0.0), None);
/// ```
pub fn flow_efficiency_percent(active_time: f64, total_elapsed_time: f64) -> Option<f64> {
    if total_elapsed_time == 0.0 {
        None
    } else {
        Some(active_time / total_elapsed_time * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Worked example: "the 'features' share of that rising velocity had
    // actually fallen from 70% to 45% over the same period".
    #[test]
    fn flow_distribution_matches_45_percent_features_share() {
        assert!((flow_distribution_percent(45.0, 100.0).unwrap() - 45.0).abs() < 1e-9);
        assert!(flow_distribution_percent(1.0, 0.0).is_none());
    }

    // Worked example: "flow load equals arrival rate multiplied by flow
    // time" (Little's law, chapter 2.4).
    #[test]
    fn littles_law_wip_equals_arrival_rate_times_flow_time() {
        assert_eq!(littles_law_wip(3.0, 8.0), 24.0);
        let recovered_flow_time = littles_law_flow_time(24.0, 3.0).unwrap();
        assert!((recovered_flow_time - 8.0).abs() < 1e-9);
        assert!(littles_law_flow_time(24.0, 0.0).is_none());
    }

    // Worked example: "flow efficiency below 25% is typical ... a change
    // spends ten hours actively being coded ... but sits idle ... across its
    // whole journey, flow efficiency is 10%" (chapter 2.5).
    #[test]
    fn flow_efficiency_of_ten_active_hours_in_a_hundred_is_ten_percent() {
        let efficiency = flow_efficiency_percent(10.0, 100.0).unwrap();
        assert!((efficiency - 10.0).abs() < 1e-9);
        assert!(flow_efficiency_percent(10.0, 0.0).is_none());
    }

    // Simple elapsed-time and count checks.
    #[test]
    fn flow_time_is_delivery_minus_entry() {
        assert_eq!(flow_time(3.0, 15.0), 12.0);
    }

    #[test]
    fn flow_load_sums_active_and_waiting() {
        assert_eq!(flow_load_from_items(5, 17), 22);
    }

    #[test]
    fn flow_velocity_divides_items_by_period() {
        assert_eq!(flow_velocity(40.0, 4.0), Some(10.0));
        assert_eq!(flow_velocity(40.0, 0.0), None);
    }
}

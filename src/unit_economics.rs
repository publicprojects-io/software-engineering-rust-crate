//! # Cost and Unit Economics of Engineering
//!
//! Engineering cost has at least three distinct components with different
//! drivers and different levers: **people cost** (salaries and benefits,
//! largely fixed in the short term), **infrastructure cost** (cloud spend,
//! largely variable with usage), and **tooling and licensing cost** (often
//! fixed per-seat or per-usage-tier). Tracking them separately, rather than
//! as one blended total, matters because a rising total driven by
//! infrastructure scaling with genuine growth calls for a very different
//! response than the same rise driven by unmanaged tooling sprawl. Dividing
//! the total by a genuine unit of value delivered — cost per customer
//! served, per transaction, per deployment — turns that total into a
//! trackable, comparable trend.
//!
//! ## Formula
//!
//! ```text
//! Total engineering cost = people cost + infrastructure cost + tooling cost
//! Unit cost                = total cost / units delivered
//! ```
//!
//! ## Why it matters
//!
//! Selecting a unit that genuinely tracks business or mission value, rather
//! than an easily inflated, internal, largely discretionary count, is what
//! keeps a unit-cost ratio honest. A single unit-cost snapshot is also less
//! useful than its trend: falling unit cost as the platform matures signals
//! genuine efficiency gains, while rising unit cost is often a direct,
//! measurable consequence of accumulated technical debt or complexity
//! hotspots elsewhere in the codebase.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::unit_economics::{total_engineering_cost, unit_cost};
//!
//! // $500k people cost, $120k infrastructure, $30k tooling, serving 10,000
//! // customers.
//! let total = total_engineering_cost(500_000.0, 120_000.0, 30_000.0);
//! assert_eq!(total, 650_000.0);
//!
//! let cost_per_customer = unit_cost(total, 10_000.0).unwrap();
//! assert_eq!(cost_per_customer, 65.0);
//! ```
//!
//! ## Money
//!
//! [`total_engineering_cost`] and [`unit_cost`] take plain `f64` amounts.
//! For currency-checked accounting, use [`rusty_money::Money`] directly
//! rather than through a wrapper this crate provides — its own `add`/`div`
//! already return `Result`, rejecting cost components quoted in different
//! currencies instead of silently treating them as the same unit:
//!
//! ```rust
//! use rusty_money::{Money, iso};
//! use software_engineering::unit_economics::unit_cost;
//!
//! let people = Money::from_major(500_000, iso::USD);
//! let infrastructure = Money::from_major(120_000, iso::USD);
//! let tooling = Money::from_major(30_000, iso::USD);
//! let total = people.add(infrastructure).unwrap().add(tooling).unwrap();
//! assert_eq!(total, Money::from_major(650_000, iso::USD));
//!
//! let cost_per_customer = unit_cost(total.to_f64_lossy(), 10_000.0).unwrap();
//! assert!((cost_per_customer - 65.0).abs() < 1e-9);
//!
//! // Dividing by zero units is rejected rather than producing infinity.
//! assert!(total.div(0).is_err());
//! ```
//!
//! ## Pitfalls
//!
//! - **Choosing an easily inflated denominator** that does not correspond to
//!   any genuine external unit of value delivered — flatters the ratio
//!   without informing anyone.
//! - **Tracking one blended cost total** instead of separating people,
//!   infrastructure, and tooling cost — hides which lever actually needs
//!   pulling when the total rises.
//! - **Reporting a unit-cost snapshot with no trend** — a single number
//!   says nothing about whether efficiency is improving or degrading.
//! - **Never connecting rising unit cost back to technical debt or
//!   complexity metrics** — misses a measurable, quantifiable case for debt
//!   remediation investment.
//!
//! ## Sources
//!
//! - Chapter 5.4, Cost and unit economics of engineering.
//! - `FinOps` Foundation, *`FinOps` Framework*.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/05-04-cost-and-unit-economics-of-engineering.md

/// The sum of engineering's three distinct cost components.
///
/// `people_cost + infrastructure_cost + tooling_cost`. This function is
/// only the sum used to compute a unit cost; callers should keep tracking
/// the three components separately elsewhere, per the chapter's explicit
/// recommendation, rather than discarding the breakdown once summed.
///
/// # Arguments
///
/// * `people_cost` — salaries and benefits, in any currency unit.
/// * `infrastructure_cost` — cloud and infrastructure spend, in the same
///   unit.
/// * `tooling_cost` — tooling and licensing cost, in the same unit.
///
/// # Returns
///
/// The total engineering cost, in the same unit.
///
/// # Examples
///
/// ```rust
/// use software_engineering::unit_economics::total_engineering_cost;
///
/// let total = total_engineering_cost(500_000.0, 120_000.0, 30_000.0);
/// assert_eq!(total, 650_000.0);
/// ```
#[must_use]
pub fn total_engineering_cost(people_cost: f64, infrastructure_cost: f64, tooling_cost: f64) -> f64 {
    people_cost + infrastructure_cost + tooling_cost
}

/// Cost per genuine unit of value delivered, such as cost per customer
/// served, per transaction, or per deployment.
///
/// `total_cost / units_delivered`.
///
/// # Arguments
///
/// * `total_cost` — total engineering cost for the period, in any currency
///   unit (typically from [`total_engineering_cost`]).
/// * `units_delivered` — count of genuine value units delivered in the same
///   period.
///
/// # Returns
///
/// The cost per unit, or `None` if `units_delivered` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::unit_economics::unit_cost;
///
/// assert_eq!(unit_cost(650_000.0, 10_000.0), Some(65.0));
/// assert_eq!(unit_cost(1.0, 0.0), None);
/// ```
#[must_use]
pub fn unit_cost(total_cost: f64, units_delivered: f64) -> Option<f64> {
    if units_delivered == 0.0 {
        return None;
    }
    Some(total_cost / units_delivered)
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Engineering cost has at least three distinct components with
    // different drivers and different levers: people cost ...
    // infrastructure cost ... and tooling and licensing cost."
    #[test]
    fn total_engineering_cost_sums_the_three_components() {
        let total = total_engineering_cost(500_000.0, 120_000.0, 30_000.0);
        assert!((total - 650_000.0).abs() < 1e-9);
    }

    // "cost per customer served, cost per transaction processed, cost per
    // deployment."
    #[test]
    fn unit_cost_divides_total_by_units_delivered() {
        let cost_per_customer = unit_cost(650_000.0, 10_000.0).unwrap();
        assert!((cost_per_customer - 65.0).abs() < 1e-9);
    }

    #[test]
    fn unit_cost_is_none_for_zero_units() {
        assert_eq!(unit_cost(1.0, 0.0), None);
    }
}

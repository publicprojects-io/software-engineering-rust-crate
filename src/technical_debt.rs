//! # Technical Debt Measurement
//!
//! **Technical debt**, a metaphor coined by Ward Cunningham, describes the
//! accumulated cost of past shortcuts — expedient decisions that shipped
//! something sooner but left the codebase harder to change afterward, in
//! the same way financial debt lets you spend now at the cost of interest
//! later. Unmeasured debt loses the prioritization competition against
//! feature work by default, not because it matters less, but because it
//! has no visible advocate; quantifying it in terms decision-makers can
//! weigh — cost to fix versus cost of carrying it — is what lets it compete
//! fairly.
//!
//! ## Formula
//!
//! ```text
//! Debt carrying cost = (velocity tax + elevated defect cost) × periods
//!
//! velocity tax          = extra cost per period from related work going slower
//! elevated defect cost  = extra expected defect cost per period from carrying the item
//! periods                = number of periods the item is left unfixed
//! ```
//!
//! ## Why it matters
//!
//! For each debt item, the chapter recommends estimating two figures: the
//! cost to fix it, and the cost of carrying it unfixed — how much slower
//! related work goes, how much additional defect risk it carries, how much
//! it blocks other work. This carrying cost, summed across however many
//! periods the item is left unaddressed, gives decision-makers a real basis
//! for comparison against feature work's cost and expected value, rather
//! than an abstract, unquantified complaint. Debt also compounds: each new
//! shortcut makes the next change slightly harder.
//!
//! ## Example
//!
//! The topic doc's enterprise example: a telecommunications company
//! allocated a fixed 15% of engineering capacity to debt remediation and
//! resolved its top five highest-carrying-cost items within a year,
//! measurably improving change failure rate for billing-related deploys —
//! the return on quantifying and targeting the highest-carrying-cost items
//! first.
//!
//! ```rust
//! use software_engineering::technical_debt::debt_carrying_cost;
//!
//! // A billing-engine shortcut: slower related work (velocity tax) plus
//! // elevated defect risk, both recurring per period, carried for a year
//! // (12 monthly periods) before remediation.
//! let cost = debt_carrying_cost(2_000.0, 500.0, 12.0);
//! assert_eq!(cost, 30_000.0);
//!
//! // Carrying the same item twice as long doubles its carrying cost.
//! let longer = debt_carrying_cost(2_000.0, 500.0, 24.0);
//! assert_eq!(longer, cost * 2.0);
//! ```
//!
//! ## Pitfalls
//!
//! - **No visible, tracked debt backlog** — debt loses the prioritization
//!   competition by default and compounds invisibly.
//! - **Vague, unquantified debt claims** — rarely compete well against
//!   concrete, quantified feature requests in planning.
//! - **Prioritizing debt by age or advocacy volume rather than impact** —
//!   misdirects limited remediation capacity away from the highest-carrying
//!   -cost items.
//! - **No protected capacity for remediation** — debt paydown only happens
//!   reactively, after a crisis, rather than as routine, deliberate practice.
//! - **Treating all debt as equally worth fixing**, instead of accepting
//!   some debt as permanent when its cost to fix exceeds its cost to carry.
//!
//! ## Sources
//!
//! - Chapter 4.5, Technical debt measurement.
//! - Cunningham, Ward, "The `WyCash` Portfolio Management System," *OOPSLA*
//!   (1992).
//! - Kruchten, Philippe, Robert Nord, and Ipek Ozkaya, *Managing Technical
//!   Debt: Reducing Friction in Software Development*.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/04-05-technical-debt-measurement.md

/// Debt carrying cost: the ongoing cost of leaving a debt item unfixed.
///
/// `(velocity_tax_per_period + elevated_defect_cost_per_period) × periods`.
/// The velocity tax captures how much slower related work goes while the
/// item is carried; the elevated defect cost captures the additional
/// expected defect risk it carries. Summing both per period gives a figure
/// decision-makers can weigh directly against the item's one-time cost to
/// fix, and against competing feature work.
///
/// # Arguments
///
/// * `velocity_tax_per_period` — extra cost per period from related work
///   going slower while the item is unfixed (any currency unit).
/// * `elevated_defect_cost_per_period` — extra expected defect cost per
///   period attributable to carrying the item, in the same unit.
/// * `periods` — number of periods (e.g. months) the item is carried
///   unfixed.
///
/// # Returns
///
/// The total carrying cost over `periods`, in the same unit as the two
/// per-period inputs.
///
/// # Examples
///
/// ```rust
/// use software_engineering::technical_debt::debt_carrying_cost;
///
/// // £2,000/month velocity tax + £500/month elevated defect cost,
/// // carried for 12 months = £30,000.
/// assert_eq!(debt_carrying_cost(2_000.0, 500.0, 12.0), 30_000.0);
/// ```
#[must_use]
pub fn debt_carrying_cost(
    velocity_tax_per_period: f64,
    elevated_defect_cost_per_period: f64,
    periods: f64,
) -> f64 {
    (velocity_tax_per_period + elevated_defect_cost_per_period) * periods
}

#[cfg(test)]
mod tests {
    use super::*;

    // "how much slower does related work go, how much additional defect
    // risk does it carry" — both costs sum per period, then scale with how
    // many periods the item is carried.
    #[test]
    fn carrying_cost_sums_velocity_tax_and_defect_cost_across_periods() {
        let cost = debt_carrying_cost(2_000.0, 500.0, 12.0);
        assert!((cost - 30_000.0).abs() < 1e-9);
    }

    // "each new shortcut makes the next change slightly harder, which
    // creates pressure for more shortcuts, which compounds further" — the
    // same per-period cost carried twice as long doubles the total.
    #[test]
    fn carrying_the_same_item_twice_as_long_doubles_its_cost() {
        let one_year = debt_carrying_cost(2_000.0, 500.0, 12.0);
        let two_years = debt_carrying_cost(2_000.0, 500.0, 24.0);
        assert!((two_years - one_year * 2.0).abs() < 1e-9);
    }

    // "allocated a fixed 15% of engineering capacity to debt remediation
    // going forward" — a zero carrying cost (no velocity tax, no elevated
    // defect cost) is the case where an item is reasonable to leave
    // permanently unaddressed, per the chapter's "accept some debt as
    // permanent" recommendation.
    #[test]
    fn zero_velocity_tax_and_defect_cost_yields_zero_carrying_cost() {
        assert!((debt_carrying_cost(0.0, 0.0, 12.0) - 0.0).abs() < 1e-9);
    }
}

//! # Test Coverage and Test Effectiveness
//!
//! **Test coverage** measures the percentage of code executed by a test
//! suite — line, branch, or path coverage. It is cheap to compute and easy
//! to visualize as a single percentage, which also makes it one of the most
//! frequently gamed metrics in software engineering: coverage measures
//! whether code executed during a test run, not whether the test actually
//! checked that the code behaved correctly. **Mutation testing** answers
//! that gap directly: it deliberately introduces small, artificial faults
//! into the code and checks whether the test suite catches ("kills") them,
//! and is coverage's necessary complement, not an optional extra.
//!
//! ## Formula
//!
//! ```text
//! Test coverage %     = lines (or branches) covered / total lines × 100
//! Mutation kill rate % = mutants killed / mutants total × 100
//!
//! covered / killed = execution or detection count
//! total            = denominator (lines, branches, or mutants); zero is undefined
//! ```
//!
//! ## Why it matters
//!
//! A coverage target with no effectiveness check is a textbook Goodhart's
//! law setup: the number improves while genuine quality does not. A test
//! suite with high line coverage but a low mutation-kill rate is executing
//! code without meaningfully checking it — pairing the two is the single
//! most effective guardrail against coverage-target gaming.
//!
//! ## Example
//!
//! The topic doc's enterprise example: a company-wide 95% coverage
//! requirement, enforced as a hard CI gate, coexisted with a mutation-kill
//! rate under 40% across much of the codebase — tests executed code without
//! meaningfully asserting on its behaviour. The revised policy required an
//! 80% mutation-kill-rate threshold for payment and authentication code.
//!
//! ```rust
//! use software_engineering::test_effectiveness::{
//!     mutation_kill_rate_percent, test_coverage_percent,
//! };
//!
//! // A company-wide "95% coverage requirement" met at face value...
//! let coverage = test_coverage_percent(95.0, 100.0);
//! assert_eq!(coverage, Some(95.0));
//!
//! // ...while the mutation-kill rate is "under 40%" on the same code —
//! // high coverage, weak verification.
//! let kill_rate = mutation_kill_rate_percent(38.0, 100.0).unwrap();
//! assert!(kill_rate < 40.0);
//!
//! // The revised policy's "80% kill-rate threshold" for critical code.
//! let critical_kill_rate = mutation_kill_rate_percent(85.0, 100.0).unwrap();
//! assert!(critical_kill_rate >= 80.0);
//!
//! // An empty denominator leaves both rates undefined.
//! assert_eq!(test_coverage_percent(1.0, 0.0), None);
//! assert_eq!(mutation_kill_rate_percent(1.0, 0.0), None);
//! ```
//!
//! ## Pitfalls
//!
//! - **Treating coverage percentage as a direct quality verdict** — it
//!   measures execution, not verification.
//! - **Writing tests primarily to satisfy a coverage gate**, producing the
//!   threshold-gaming pattern a coverage-only target invites.
//! - **Disabling or deleting failing tests** instead of fixing the
//!   underlying problem, which removes real protection while the reported
//!   number barely moves.
//! - **Applying a uniform coverage target regardless of code risk**, wasting
//!   effort on low-risk code and under-investing in critical paths.
//! - **Treating a coverage-mutation gap as evidence of nothing**: a large
//!   gap between a high coverage number and a low mutation-kill rate is the
//!   clearest sign that coverage alone is not telling you what you think.
//!
//! ## Sources
//!
//! - Chapter 4.2, Test coverage and test effectiveness.
//! - Jia, Yue, and Mark Harman, "An Analysis and Survey of the Development
//!   of Mutation Testing," *IEEE Transactions on Software Engineering*
//!   (2011).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/04-02-test-coverage-and-test-effectiveness.md

/// Test coverage as a percentage: covered lines (or branches) / total × 100.
///
/// Coverage measures whether code executed during a test run, not whether
/// the test meaningfully verified its behaviour. Use it to find untested
/// code (a floor), not as a ceiling to maximize; pair it with
/// [`mutation_kill_rate_percent`] to check that the covered code is actually
/// being verified.
///
/// # Arguments
///
/// * `covered` — lines, branches, or paths executed by the test suite.
/// * `total` — total lines, branches, or paths in the measured code.
///
/// # Returns
///
/// `Some(percentage)` (e.g. `95.0` for 95%), or `None` when `total` is zero
/// (coverage undefined — there is no code to cover).
///
/// # Examples
///
/// ```rust
/// use software_engineering::test_effectiveness::test_coverage_percent;
///
/// // A company-wide 95% coverage requirement, met at face value.
/// assert_eq!(test_coverage_percent(95.0, 100.0), Some(95.0));
/// assert_eq!(test_coverage_percent(1.0, 0.0), None);
/// ```
#[must_use]
pub fn test_coverage_percent(covered: f64, total: f64) -> Option<f64> {
    if total == 0.0 {
        None
    } else {
        Some(covered / total * 100.0)
    }
}

/// Mutation kill rate as a percentage: mutants killed / mutants total × 100.
///
/// Mutation testing introduces small, artificial faults (flipping a
/// comparison operator, changing a boundary condition) and checks whether
/// the test suite fails against each mutated version. A high kill rate
/// means the tests are genuinely verifying behaviour, not merely executing
/// it; this is the check on test effectiveness itself, and coverage's
/// necessary complement.
///
/// # Arguments
///
/// * `mutants_killed` — mutants the test suite detected (caused a failure).
/// * `mutants_total` — total mutants generated and run against the suite.
///
/// # Returns
///
/// `Some(percentage)`, or `None` when `mutants_total` is zero (no mutants
/// were generated — kill rate undefined).
///
/// # Examples
///
/// ```rust
/// use software_engineering::test_effectiveness::mutation_kill_rate_percent;
///
/// // High coverage paired with a kill rate "under 40%": tests execute
/// // code without meaningfully checking it.
/// let kill_rate = mutation_kill_rate_percent(38.0, 100.0).unwrap();
/// assert!(kill_rate < 40.0);
/// assert_eq!(mutation_kill_rate_percent(1.0, 0.0), None);
/// ```
#[must_use]
pub fn mutation_kill_rate_percent(mutants_killed: f64, mutants_total: f64) -> Option<f64> {
    if mutants_total == 0.0 {
        None
    } else {
        Some(mutants_killed / mutants_total * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // "leadership had set a company-wide 95% coverage requirement for all
    // new code, enforced as a hard CI gate."
    #[test]
    fn company_wide_coverage_requirement_is_95_percent() {
        assert!((test_coverage_percent(95.0, 100.0).unwrap() - 95.0).abs() < 1e-9);
    }

    // "found a mutation-kill rate under 40% across much of the codebase."
    #[test]
    fn mutation_kill_rate_can_be_under_40_percent_despite_high_coverage() {
        let kill_rate = mutation_kill_rate_percent(38.0, 100.0).unwrap();
        assert!(kill_rate < 40.0);
    }

    // "mandatory mutation testing above an 80% kill-rate threshold for
    // payment and authentication code."
    #[test]
    fn critical_code_policy_requires_at_least_80_percent_kill_rate() {
        let kill_rate = mutation_kill_rate_percent(85.0, 100.0).unwrap();
        assert!(kill_rate >= 80.0);
    }

    // Coverage measures execution over a total; a zero total (no code to
    // cover) leaves the percentage undefined.
    #[test]
    fn coverage_is_none_when_total_is_zero() {
        assert!(test_coverage_percent(1.0, 0.0).is_none());
    }

    // "a test suite with high line coverage but a low mutation-kill rate is
    // executing code without meaningfully checking it" — a zero mutant
    // count is equally undefined, not zero.
    #[test]
    fn mutation_kill_rate_is_none_when_no_mutants_were_run() {
        assert!(mutation_kill_rate_percent(1.0, 0.0).is_none());
    }
}

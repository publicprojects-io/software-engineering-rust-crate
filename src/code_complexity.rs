//! # Code Complexity Metrics
//!
//! **Cyclomatic complexity**, introduced by Thomas J. `McCabe` in 1976, counts
//! the number of independent paths through a piece of code's control flow:
//! each `if`, loop, and branch adds to the count. Code with more independent
//! paths through it is harder to fully test, harder to reason about, and,
//! in decades of empirical research, measurably more likely to contain
//! defects.
//!
//! ## Formula
//!
//! ```text
//! Cyclomatic complexity = edges - nodes + 2
//!
//! edges = control-flow graph edges
//! nodes = control-flow graph nodes
//! ```
//!
//! ## Why it matters
//!
//! Complexity metrics predict testing and defect difficulty; they do not
//! measure quality directly. For large teams they earn their keep as a
//! triage tool: a way to find, among thousands of files, the small subset
//! most likely to reward a closer look, not as a standalone verdict on code
//! quality.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::code_complexity::cyclomatic_complexity;
//!
//! // A straight-line function with no branches: 1 node, 0 back-edges
//! // beyond the single entry/exit edge — McCabe's minimum score is 1.
//! // Graph: 2 nodes (entry, exit), 1 edge: 1 - 2 + 2 = 1.
//! assert_eq!(cyclomatic_complexity(1, 2), 1);
//!
//! // A single `if` adds one more independent path: 3 edges, 3 nodes.
//! assert_eq!(cyclomatic_complexity(3, 3), 2);
//! ```
//!
//! ## Pitfalls
//!
//! - **Treating a complexity score as a direct quality verdict** — it
//!   measures one specific property, not overall code quality.
//! - **Decomposition gaming**: splitting a function to lower the score
//!   without genuinely simplifying anything, sometimes scattering the logic
//!   across more files and making it harder to follow.
//! - **Applying a universal threshold without calibrating to your own
//!   codebase** — a parser or rules engine may have legitimately higher
//!   baseline complexity than a typical CRUD service.
//! - **Using complexity metrics to individually evaluate engineers**
//!   invites gaming and misapplies a metric meant for triage, not judgement.
//!
//! ## Sources
//!
//! - Chapter 4.1, Code complexity metrics.
//! - `McCabe`, Thomas J., "A Complexity Measure," *IEEE Transactions on
//!   Software Engineering* (1976).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/04-01-code-complexity-metrics.md

/// `McCabe` cyclomatic complexity: independent paths through control flow.
///
/// `edges - nodes + 2`, computed on the function's control-flow graph. A
/// straight-line function with no branches scores 1 (the minimum); each
/// additional decision point (`if`, loop, `case` arm, and similar) adds one.
///
/// # Arguments
///
/// * `edges` — number of edges in the control-flow graph.
/// * `nodes` — number of nodes in the control-flow graph.
///
/// # Returns
///
/// The cyclomatic complexity score (an integer; can be negative for a
/// malformed or disconnected graph, which the caller should treat as
/// invalid input).
///
/// # Examples
///
/// ```rust
/// use software_engineering::code_complexity::cyclomatic_complexity;
///
/// // Straight-line function: 2 nodes, 1 edge -> complexity 1.
/// assert_eq!(cyclomatic_complexity(1, 2), 1);
/// ```
#[must_use]
pub fn cyclomatic_complexity(edges: i64, nodes: i64) -> i64 {
    edges - nodes + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // "each `if`, loop, and branch adds to the count" — a straight-line
    // function (no branches) has McCabe's documented minimum score of 1.
    #[test]
    fn straight_line_function_has_complexity_one() {
        assert_eq!(cyclomatic_complexity(1, 2), 1);
    }

    // Adding one decision point (one `if`) adds exactly one independent
    // path, per McCabe's formula.
    #[test]
    fn single_if_branch_adds_one_independent_path() {
        assert_eq!(cyclomatic_complexity(3, 3), 2);
    }

    // "a single transaction-validation function with a cyclomatic
    // complexity score more than ten times the codebase's median" — a
    // sanity check that larger graphs produce proportionally larger scores.
    #[test]
    fn more_decision_points_yield_higher_complexity() {
        let simple = cyclomatic_complexity(1, 2);
        let complex = cyclomatic_complexity(21, 12); // 11 decision points
        assert!(complex > simple * 10);
    }
}

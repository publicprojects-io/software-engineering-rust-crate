//! # Communication and Collaboration Metrics
//!
//! **Communication and collaboration**, the C in SPACE (chapter 3.1),
//! measures how information actually flows between people and teams: how
//! discoverable documentation is, how evenly knowledge spreads, how well
//! cross-team dependencies get coordinated, and how new team members
//! onboard into shared understanding. This dimension is often the least
//! instrumented of the five, precisely because it is harder to observe
//! than delivery data — and that gap matters, because breakdowns here
//! frequently show up, misattributed, in every other dimension: a rising
//! change failure rate that looks like a testing problem is sometimes
//! actually a communication problem, a team that did not know about a
//! dependency's change until it broke in production.
//!
//! ## Formula
//!
//! ```text
//! Cross-team dependency resolution time = resolved at − raised at
//!     (a request, API change, or coordinated release, team-to-team)
//!
//! Time to first contribution = first contribution at − joined at
//!     (onboarding proxy for how well shared understanding flows)
//! ```
//!
//! ## Why it matters
//!
//! A team that consistently waits weeks for a dependency another team owns
//! has a collaboration problem that will not show up cleanly in either
//! team's own internal delivery metrics — it needs a direct,
//! inter-team-specific signal, the same cycle-time discipline chapter 2.6
//! applies within a team, applied here across a team boundary instead. The
//! time from a new team member joining to their first meaningful,
//! independent contribution is a complementary, practical proxy for the
//! same underlying property: a team where knowledge lives entirely in
//! people's heads onboards slowly and unpredictably, while a team with
//! genuinely good documentation, clear ownership, and accessible
//! mentorship onboards faster and more consistently.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::communication_metrics::{
//!     cross_team_dependency_resolution_time_days, time_to_first_contribution_days,
//! };
//!
//! // A shared-library change request raised on day 10, resolved on day 24:
//! // two weeks of inter-team coordination friction.
//! let resolution_time = cross_team_dependency_resolution_time_days(10.0, 24.0);
//! assert_eq!(resolution_time, 14.0);
//!
//! // A new engineer joins on day 0 and lands their first independent
//! // change on day 18.
//! let onboarding_time = time_to_first_contribution_days(0.0, 18.0);
//! assert_eq!(onboarding_time, 18.0);
//! ```
//!
//! ## Pitfalls
//!
//! - **Measuring only intra-team cycle time** — a slow cross-team
//!   dependency will not show up in either owning team's own internal
//!   delivery metrics; it needs its own direct signal.
//! - **Treating documentation existence as sufficient** — track
//!   discoverability (is it actually found and used), not just whether
//!   content technically exists somewhere (chapter 4.6).
//! - **Ignoring onboarding time as "just an HR concern"** — a long or
//!   highly variable time to first contribution is a genuine collaboration
//!   signal about how well shared understanding flows.
//! - **Assuming the org chart describes real communication** — periodic,
//!   lightweight analysis of who actually collaborates with whom (code
//!   review networks, meeting overlap) often reveals a bottleneck or an
//!   isolated pocket the formal structure hides.
//!
//! ## Sources
//!
//! - Chapter 3.5, Communication and collaboration metrics.
//! - Forsgren, Storey, Maddila, Zimmermann, Houck, and Butler, "The SPACE of
//!   Developer Productivity," *ACM Queue* (2021).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/03-05-communication-and-collaboration-metrics.md

/// Time from a cross-team dependency being raised to being resolved: a
/// shared-library update, an API change request, a coordinated release,
/// team-to-team rather than within a single team.
///
/// `resolved_at_days − raised_at_days`.
///
/// # Arguments
///
/// * `raised_at_days` — when the cross-team request was raised.
/// * `resolved_at_days` — when the other team resolved it.
///
/// # Returns
///
/// The resolution time in days.
///
/// # Examples
///
/// ```rust
/// use software_engineering::communication_metrics::cross_team_dependency_resolution_time_days;
///
/// assert_eq!(cross_team_dependency_resolution_time_days(10.0, 24.0), 14.0);
/// ```
#[must_use]
pub fn cross_team_dependency_resolution_time_days(raised_at_days: f64, resolved_at_days: f64) -> f64 {
    resolved_at_days - raised_at_days
}

/// Time from a new team member joining to their first meaningful,
/// independent contribution — a practical proxy for how well shared
/// understanding flows in an organization.
///
/// `first_contribution_at_days − joined_at_days`.
///
/// # Arguments
///
/// * `joined_at_days` — when the new team member joined.
/// * `first_contribution_at_days` — when they made their first meaningful,
///   independent contribution.
///
/// # Returns
///
/// The time to first contribution, in days.
///
/// # Examples
///
/// ```rust
/// use software_engineering::communication_metrics::time_to_first_contribution_days;
///
/// assert_eq!(time_to_first_contribution_days(0.0, 18.0), 18.0);
/// ```
#[must_use]
pub fn time_to_first_contribution_days(joined_at_days: f64, first_contribution_at_days: f64) -> f64 {
    first_contribution_at_days - joined_at_days
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Track how long a cross-team request ... takes from being raised to
    // being resolved ... applied specifically to inter-team, rather than
    // intra-team, coordination."
    #[test]
    fn cross_team_resolution_time_is_resolved_minus_raised() {
        let time = cross_team_dependency_resolution_time_days(10.0, 24.0);
        assert!((time - 14.0).abs() < 1e-9);
    }

    #[test]
    fn cross_team_resolution_time_can_be_same_day() {
        let time = cross_team_dependency_resolution_time_days(5.0, 5.0);
        assert!((time - 0.0).abs() < 1e-9);
    }

    // "The time from a new team member joining to their first meaningful,
    // independent contribution is a strong, practical proxy for how well
    // shared understanding actually flows."
    #[test]
    fn time_to_first_contribution_is_first_contribution_minus_joined() {
        let time = time_to_first_contribution_days(0.0, 18.0);
        assert!((time - 18.0).abs() < 1e-9);
    }

    #[test]
    fn time_to_first_contribution_from_a_nonzero_start_date() {
        let time = time_to_first_contribution_days(100.0, 112.0);
        assert!((time - 12.0).abs() < 1e-9);
    }
}

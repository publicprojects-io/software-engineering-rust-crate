//! # The DORA Metrics Framework
//!
//! The **DORA metrics** come from the DevOps Research and Assessment
//! programme, later published as the book *Accelerate*, which surveyed tens
//! of thousands of engineering professionals to find which delivery
//! practices correlate with organizational performance. Four metrics, paired
//! two and two: **deployment frequency** and **lead time for changes**
//! measure speed; **change failure rate** and **failed deployment recovery
//! time** measure stability. The framework's central finding is that elite
//! performers are fast and stable simultaneously — speed and safety do not
//! trade off against each other the way intuition suggests.
//!
//! ## Formula
//!
//! ```text
//! Deployment frequency         = deployments / days
//! Lead time for changes        = deploy time − first commit time
//! Change failure rate (%)      = (failed deployments / total deployments) × 100
//! Failed deployment recovery   = restored time − detected time (never the deploy event)
//! ```
//!
//! ## Why it matters
//!
//! DORA measures the pipeline, not the value flowing through it: a team can
//! post excellent DORA numbers while its actual output has quietly drifted
//! toward rework, a gap this book's Flow Framework chapters are built to
//! surface and DORA cannot see. Used within that bounded scope, DORA gives
//! large organizations a consistent, comparable measure of pipeline
//! mechanics across many teams — genuinely valuable for prioritizing
//! platform investment, provided all four metrics are reported together and
//! never applied to individual performance reviews.
//!
//! ## Example
//!
//! A platform team's change failure rate falls from 25% to 8% across 100
//! production deployments a year, a stability improvement that DORA's
//! pairing discipline insists on measuring alongside any speed gain, never
//! in isolation.
//!
//! ```rust
//! use software_engineering::dora_metrics::{
//!     change_failure_rate_percent, deployment_frequency_per_day,
//!     lead_time_for_changes_hours, failed_deployment_recovery_time_hours,
//! };
//!
//! let cfr_before = change_failure_rate_percent(25.0, 100.0).unwrap();
//! let cfr_after = change_failure_rate_percent(8.0, 100.0).unwrap();
//! assert_eq!(cfr_before, 25.0);
//! assert_eq!(cfr_after, 8.0);
//! assert!(cfr_after < cfr_before);
//!
//! // 2 deployments/day, a 6-hour lead time from first commit to production,
//! // and a 1.5-hour recovery from detection to restoration.
//! assert_eq!(deployment_frequency_per_day(14.0, 7.0).unwrap(), 2.0);
//! assert_eq!(lead_time_for_changes_hours(0.0, 6.0), 6.0);
//! assert_eq!(failed_deployment_recovery_time_hours(10.0, 11.5), 1.5);
//! ```
//!
//! ## Pitfalls
//!
//! - **Treating DORA as the whole picture of delivery health** — it is
//!   silent on what kind of value is being delivered; pair it with flow
//!   distribution.
//! - **Reporting only the speed half** — defeats the framework's central
//!   finding that speed and stability move together in high performers.
//! - **Using DORA metrics in individual performance reviews** — breaks the
//!   framework's statistical validity and invites gaming.
//! - **Comparing teams with inconsistent definitions** of "deployment,"
//!   "change," or "failure" — produces comparisons that look fair but are
//!   not.
//! - **Self-reported DORA numbers instead of pipeline-instrumented ones** —
//!   reintroduces exactly the bias the framework was designed to eliminate.
//!
//! ## Sources
//!
//! - Chapter 2.10, The DORA metrics framework.
//! - Forsgren, Nicole, Jez Humble, and Gene Kim, *Accelerate: The Science of
//!   Lean Software and DevOps* (2018).
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/02-10-the-dora-metrics-framework.md

/// Change failure rate: the percentage of deployments that caused a failure
/// requiring remediation, a rollback, a hotfix, or an incident.
///
/// `(failed_deployments / total_deployments) × 100`. Definition drift here
/// is the chapter's specific warning: agree on what counts as a "failure" in
/// writing before comparing this number across teams.
///
/// # Arguments
///
/// * `failed_deployments` — count of deployments that caused a failure.
/// * `total_deployments` — total count of deployments in the period.
///
/// # Returns
///
/// The failure rate as a percentage (0.0–100.0 for sane inputs), or `None`
/// if `total_deployments` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::dora_metrics::change_failure_rate_percent;
///
/// assert_eq!(change_failure_rate_percent(25.0, 100.0), Some(25.0));
/// assert_eq!(change_failure_rate_percent(8.0, 100.0), Some(8.0));
/// assert_eq!(change_failure_rate_percent(1.0, 0.0), None);
/// ```
#[must_use]
pub fn change_failure_rate_percent(failed_deployments: f64, total_deployments: f64) -> Option<f64> {
    if total_deployments == 0.0 {
        return None;
    }
    Some((failed_deployments / total_deployments) * 100.0)
}

/// Deployment frequency: how often a team successfully releases to
/// production.
///
/// `deployments / days`. Count only successful production deployments,
/// instrumented from the pipeline, never self-reported, and watch for
/// substitution gaming — splitting one meaningful change into several
/// trivial deploys purely to inflate the count.
///
/// # Arguments
///
/// * `deployments` — count of successful production deployments.
/// * `days` — length of the observation period, in days.
///
/// # Returns
///
/// Deployments per day, or `None` if `days` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::dora_metrics::deployment_frequency_per_day;
///
/// // 14 deployments across a week is 2 per day.
/// assert_eq!(deployment_frequency_per_day(14.0, 7.0), Some(2.0));
/// assert_eq!(deployment_frequency_per_day(1.0, 0.0), None);
/// ```
#[must_use]
pub fn deployment_frequency_per_day(deployments: f64, days: f64) -> Option<f64> {
    if days == 0.0 {
        return None;
    }
    Some(deployments / days)
}

/// Lead time for changes: the time from a code change's first commit to its
/// successful deployment in production.
///
/// `deploy_time_hours − first_commit_time_hours`. Report both the median and
/// a high percentile across many changes, not just a mean, since this
/// quantity is typically skewed.
///
/// # Arguments
///
/// * `first_commit_time_hours` — timestamp of the change's first commit, in
///   hours on any consistent scale.
/// * `deploy_time_hours` — timestamp of its successful production
///   deployment, on the same scale.
///
/// # Returns
///
/// The elapsed lead time in hours.
///
/// # Examples
///
/// ```rust
/// use software_engineering::dora_metrics::lead_time_for_changes_hours;
///
/// // First commit at hour 0, deployed at hour 6: a 6-hour lead time.
/// assert_eq!(lead_time_for_changes_hours(0.0, 6.0), 6.0);
/// ```
#[must_use]
pub fn lead_time_for_changes_hours(first_commit_time_hours: f64, deploy_time_hours: f64) -> f64 {
    deploy_time_hours - first_commit_time_hours
}

/// Failed deployment recovery time (often shortened to MTTR): how long it
/// takes to restore service once a deployment causes a failure.
///
/// `restored_time_hours − detected_time_hours`. Start the clock at
/// detection, not at the deploy event itself, so the number reflects
/// genuine recovery delay rather than a monitoring gap.
///
/// # Arguments
///
/// * `detected_time_hours` — timestamp the failure was detected, in hours on
///   any consistent scale.
/// * `restored_time_hours` — timestamp service was restored, on the same
///   scale.
///
/// # Returns
///
/// The elapsed recovery time in hours.
///
/// # Examples
///
/// ```rust
/// use software_engineering::dora_metrics::failed_deployment_recovery_time_hours;
///
/// // Detected at hour 10, restored at hour 11.5: a 1.5-hour recovery.
/// assert_eq!(failed_deployment_recovery_time_hours(10.0, 11.5), 1.5);
/// ```
#[must_use]
pub fn failed_deployment_recovery_time_hours(detected_time_hours: f64, restored_time_hours: f64) -> f64 {
    restored_time_hours - detected_time_hours
}

#[cfg(test)]
mod tests {
    use super::*;

    // "Change failure rate measures the percentage of deployments that
    // cause a failure requiring remediation, a rollback, a hotfix, or an
    // incident."
    #[test]
    fn change_failure_rate_computes_percentage() {
        let cfr = change_failure_rate_percent(25.0, 100.0).unwrap();
        assert!((cfr - 25.0).abs() < 1e-9);
        let cfr_after = change_failure_rate_percent(8.0, 100.0).unwrap();
        assert!((cfr_after - 8.0).abs() < 1e-9);
    }

    #[test]
    fn change_failure_rate_is_none_for_zero_total() {
        assert_eq!(change_failure_rate_percent(1.0, 0.0), None);
    }

    // "Deployment frequency measures how often a team successfully
    // releases to production."
    #[test]
    fn deployment_frequency_computes_rate_per_day() {
        let freq = deployment_frequency_per_day(14.0, 7.0).unwrap();
        assert!((freq - 2.0).abs() < 1e-9);
    }

    #[test]
    fn deployment_frequency_is_none_for_zero_days() {
        assert_eq!(deployment_frequency_per_day(1.0, 0.0), None);
    }

    // "Lead time for changes measures the time from a code change's first
    // commit to its successful deployment in production."
    #[test]
    fn lead_time_is_deploy_time_minus_first_commit_time() {
        let lead_time = lead_time_for_changes_hours(2.0, 8.0);
        assert!((lead_time - 6.0).abs() < 1e-9);
    }

    // "Start the clock at detection, not at the deploy event itself, so the
    // number reflects genuine recovery delay rather than a monitoring gap."
    #[test]
    fn recovery_time_starts_at_detection_not_deploy() {
        let recovery = failed_deployment_recovery_time_hours(10.0, 11.5);
        assert!((recovery - 1.5).abs() < 1e-9);
    }
}

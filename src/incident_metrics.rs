//! # Incident Metrics
//!
//! Incident response time decomposes into distinct phases: **mean time to
//! detect (MTTD)**, how long before the organization notices something is
//! wrong; **mean time to acknowledge (MTTA)**, how long before someone takes
//! ownership of responding; and **mean time to resolve or recover (MTTR)**,
//! how long from ownership to genuine recovery. Reporting these separately,
//! rather than only a single blended total, matters because each phase
//! points to a different fix: slow detection points to a monitoring gap,
//! slow acknowledgement points to an on-call process gap, and slow
//! resolution points to a tooling or runbook gap.
//!
//! ## Formula
//!
//! ```text
//! MTTD          = mean(detection durations)
//! MTTA          = mean(acknowledgement durations)
//! MTTR          = mean(resolution durations)
//! Mean duration = mean(total incident durations)
//! ```
//!
//! ## Why it matters
//!
//! Track incident frequency and MTTR together, never in isolation, mirroring
//! DORA's speed-and-stability pairing discipline: an improving MTTR
//! alongside a rising incident frequency might indicate a team getting
//! better at firefighting while underlying reliability actually degrades, and
//! a falling frequency alongside a worsening MTTR might indicate rarer but
//! more severe, harder-to-diagnose failures replacing frequent minor ones.
//! Reviewing both together, rather than either alone, is what gives an
//! honest combined picture.
//!
//! ## Example
//!
//! ```rust
//! use software_engineering::incident_metrics::{
//!     mean_time_to_detect_minutes, mean_time_to_acknowledge_minutes,
//!     mean_time_to_resolve_minutes, mean_incident_duration_minutes,
//! };
//!
//! let detection = [5.0, 15.0];
//! let acknowledgement = [2.0, 4.0];
//! let resolution = [30.0, 90.0];
//! let total = [37.0, 109.0];
//!
//! assert_eq!(mean_time_to_detect_minutes(&detection), Some(10.0));
//! assert_eq!(mean_time_to_acknowledge_minutes(&acknowledgement), Some(3.0));
//! assert_eq!(mean_time_to_resolve_minutes(&resolution), Some(60.0));
//! assert_eq!(mean_incident_duration_minutes(&total), Some(73.0));
//! ```
//!
//! ## Pitfalls
//!
//! - **Reporting only a single blended total** instead of the three
//!   decomposed phases — hides which specific gap (monitoring, on-call
//!   process, or tooling) is driving a slow response.
//! - **Reviewing incident frequency and MTTR in isolation** — misses the
//!   pattern where one metric's improvement masks the other's decline.
//! - **Inconsistent severity classification across teams** — makes
//!   organization-wide incident data as unreliable for comparison as
//!   inconsistently classified defect data.
//! - **Extracting no systemic action items from postmortems** — produces
//!   insight with no follow-through, wasting the organizational learning
//!   the process is meant to capture.
//!
//! ## Sources
//!
//! - Chapter 6.2, Incident metrics.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/06-02-incident-metrics.md

/// The arithmetic mean of a slice of per-incident durations, in minutes.
///
/// Shared by all four public functions in this module, each of which
/// applies it to a different phase of incident response.
fn mean_duration_minutes(durations_minutes: &[f64]) -> Option<f64> {
    if durations_minutes.is_empty() {
        return None;
    }
    let sum: f64 = durations_minutes.iter().sum();
    // Incident counts never approach f64's precision limit, so this cast
    // never loses precision in practice.
    #[allow(clippy::cast_precision_loss)]
    let count = durations_minutes.len() as f64;
    Some(sum / count)
}

/// Mean time to detect (MTTD): the mean, across incidents, of the duration
/// from a failure's actual onset to someone noticing it.
///
/// # Arguments
///
/// * `detection_durations_minutes` — per-incident detection durations, in
///   minutes.
///
/// # Returns
///
/// The mean detection duration, or `None` if the slice is empty.
///
/// # Examples
///
/// ```rust
/// use software_engineering::incident_metrics::mean_time_to_detect_minutes;
///
/// assert_eq!(mean_time_to_detect_minutes(&[5.0, 15.0]), Some(10.0));
/// assert_eq!(mean_time_to_detect_minutes(&[]), None);
/// ```
#[must_use]
pub fn mean_time_to_detect_minutes(detection_durations_minutes: &[f64]) -> Option<f64> {
    mean_duration_minutes(detection_durations_minutes)
}

/// Mean time to acknowledge (MTTA): the mean, across incidents, of the
/// duration from notification to someone taking ownership of the response.
///
/// # Arguments
///
/// * `acknowledgement_durations_minutes` — per-incident acknowledgement
///   durations, in minutes.
///
/// # Returns
///
/// The mean acknowledgement duration, or `None` if the slice is empty.
///
/// # Examples
///
/// ```rust
/// use software_engineering::incident_metrics::mean_time_to_acknowledge_minutes;
///
/// assert_eq!(mean_time_to_acknowledge_minutes(&[2.0, 4.0]), Some(3.0));
/// assert_eq!(mean_time_to_acknowledge_minutes(&[]), None);
/// ```
#[must_use]
pub fn mean_time_to_acknowledge_minutes(acknowledgement_durations_minutes: &[f64]) -> Option<f64> {
    mean_duration_minutes(acknowledgement_durations_minutes)
}

/// Mean time to resolve or recover (MTTR): the mean, across incidents, of
/// the duration from ownership to genuine recovery.
///
/// # Arguments
///
/// * `resolution_durations_minutes` — per-incident resolution durations, in
///   minutes.
///
/// # Returns
///
/// The mean resolution duration, or `None` if the slice is empty.
///
/// # Examples
///
/// ```rust
/// use software_engineering::incident_metrics::mean_time_to_resolve_minutes;
///
/// assert_eq!(mean_time_to_resolve_minutes(&[30.0, 90.0]), Some(60.0));
/// assert_eq!(mean_time_to_resolve_minutes(&[]), None);
/// ```
#[must_use]
pub fn mean_time_to_resolve_minutes(resolution_durations_minutes: &[f64]) -> Option<f64> {
    mean_duration_minutes(resolution_durations_minutes)
}

/// Mean total incident duration, from detection start to full resolution —
/// the blended total to report *alongside*, never instead of, the three
/// decomposed phases above.
///
/// # Arguments
///
/// * `total_durations_minutes` — per-incident total durations, in minutes.
///
/// # Returns
///
/// The mean total duration, or `None` if the slice is empty.
///
/// # Examples
///
/// ```rust
/// use software_engineering::incident_metrics::mean_incident_duration_minutes;
///
/// assert_eq!(mean_incident_duration_minutes(&[37.0, 109.0]), Some(73.0));
/// assert_eq!(mean_incident_duration_minutes(&[]), None);
/// ```
#[must_use]
pub fn mean_incident_duration_minutes(total_durations_minutes: &[f64]) -> Option<f64> {
    mean_duration_minutes(total_durations_minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    // "mean time to detect (MTTD), how long before the organization
    // notices something is wrong."
    #[test]
    fn mttd_computes_mean_of_detection_durations() {
        let mttd = mean_time_to_detect_minutes(&[5.0, 15.0]).unwrap();
        assert!((mttd - 10.0).abs() < 1e-9);
    }

    #[test]
    fn mttd_is_none_for_empty_slice() {
        assert_eq!(mean_time_to_detect_minutes(&[]), None);
    }

    // "mean time to acknowledge (MTTA), how long before someone takes
    // ownership of responding."
    #[test]
    fn mtta_computes_mean_of_acknowledgement_durations() {
        let mtta = mean_time_to_acknowledge_minutes(&[2.0, 4.0]).unwrap();
        assert!((mtta - 3.0).abs() < 1e-9);
    }

    #[test]
    fn mtta_is_none_for_empty_slice() {
        assert_eq!(mean_time_to_acknowledge_minutes(&[]), None);
    }

    // "mean time to resolve or recover (MTTR), how long from ownership to
    // genuine recovery."
    #[test]
    fn mttr_computes_mean_of_resolution_durations() {
        let mttr = mean_time_to_resolve_minutes(&[30.0, 90.0]).unwrap();
        assert!((mttr - 60.0).abs() < 1e-9);
    }

    #[test]
    fn mttr_is_none_for_empty_slice() {
        assert_eq!(mean_time_to_resolve_minutes(&[]), None);
    }

    #[test]
    fn mean_incident_duration_computes_mean_of_totals() {
        let mean_duration = mean_incident_duration_minutes(&[37.0, 109.0]).unwrap();
        assert!((mean_duration - 73.0).abs() < 1e-9);
    }

    #[test]
    fn mean_incident_duration_is_none_for_empty_slice() {
        assert_eq!(mean_incident_duration_minutes(&[]), None);
    }
}

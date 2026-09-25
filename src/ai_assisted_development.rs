//! # Measuring AI-Assisted Software Development
//!
//! AI coding assistance can feel dramatically faster at the point of initial
//! code generation while showing no net cycle-time improvement once the
//! full pipeline, including review and correction, is measured — code that
//! is faster to produce but slower to review, or that requires more rework,
//! can offset or reverse the apparent gain. A genuine productivity gain
//! shows faster cycle time *with* stable or improved quality; a false gain
//! shows faster cycle time *with* degrading quality, exactly the trade this
//! book warns against throughout, discovered here through the same
//! paired-metric discipline applied elsewhere to DORA and to flow metrics.
//!
//! ## Formula
//!
//! ```text
//! Net cycle-time change (%) = (cycle time before − cycle time after) / cycle time before × 100
//! Genuine gain                 when cycle time improved AND defect rate did not worsen
//! ```
//!
//! ## Why it matters
//!
//! Developer self-report of "this saved me an hour" is a useful starting
//! hypothesis, subject to the same recall and desirability biases as any
//! self-reported data, and it says nothing about downstream review or
//! correction cost. Measuring the full cycle-time chain, not just the
//! coding stage, and checking it against change failure rate or escaped
//! defect rate rather than trusting a felt sense of speed, is what
//! distinguishes a genuine gain from a false one.
//!
//! ## Example
//!
//! A team's initial code-generation step feels dramatically faster, but the
//! full pipeline, including a slower review and correction step, shows no
//! net cycle-time improvement, and the defect rate has quietly worsened —
//! the chapter's named false-gain pattern.
//!
//! ```rust
//! use software_engineering::ai_assisted_development::{
//!     net_cycle_time_change_percent, is_genuine_productivity_gain,
//! };
//!
//! // Full cycle time (generation + review + correction) fell from 10 hours
//! // to 8 hours: a 20% improvement.
//! let change = net_cycle_time_change_percent(10.0, 8.0).unwrap();
//! assert!((change - 20.0).abs() < 1e-9);
//!
//! // But if the defect rate rose from 2% to 5% alongside that speedup,
//! // this is a false gain, not a genuine one.
//! assert!(!is_genuine_productivity_gain(10.0, 8.0, 2.0, 5.0));
//!
//! // The same speedup with a stable or improved defect rate is genuine.
//! assert!(is_genuine_productivity_gain(10.0, 8.0, 2.0, 2.0));
//! ```
//!
//! ## Pitfalls
//!
//! - **Measuring only the generation-speed step**, ignoring full cycle time
//!   — the chapter's central named pitfall; review and correction cost can
//!   offset or reverse the apparent gain entirely.
//! - **Treating self-reported time savings as a conclusion** rather than a
//!   starting hypothesis to validate against objective cycle-time and
//!   quality data.
//! - **Comparing only a before-and-after snapshot**, without a genuine
//!   comparison group or a longer historical baseline — cannot distinguish
//!   AI assistance's effect from any other concurrent change.
//! - **Reporting a single blended average across task types** — hides that
//!   assistance may provide strong value for boilerplate work and little or
//!   negative value for genuinely novel, complex problem-solving.
//!
//! ## Sources
//!
//! - Chapter 7.2, Measuring AI-assisted software development.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/07-02-measuring-ai-assisted-software-development.md

/// Net cycle-time change: the percentage change in full cycle time
/// (generation plus review plus correction), positive meaning faster.
///
/// `(cycle_time_before_hours − cycle_time_after_hours) / cycle_time_before_hours
/// × 100`.
///
/// # Arguments
///
/// * `cycle_time_before_hours` — full cycle time before AI assistance, in
///   hours.
/// * `cycle_time_after_hours` — full cycle time after AI assistance, in
///   hours, measured across the same full pipeline.
///
/// # Returns
///
/// The percentage change (positive is faster, negative is slower), or
/// `None` if `cycle_time_before_hours` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::ai_assisted_development::net_cycle_time_change_percent;
///
/// assert!((net_cycle_time_change_percent(10.0, 8.0).unwrap() - 20.0).abs() < 1e-9);
/// assert_eq!(net_cycle_time_change_percent(0.0, 1.0), None);
/// ```
#[must_use]
pub fn net_cycle_time_change_percent(cycle_time_before_hours: f64, cycle_time_after_hours: f64) -> Option<f64> {
    if cycle_time_before_hours == 0.0 {
        return None;
    }
    Some(((cycle_time_before_hours - cycle_time_after_hours) / cycle_time_before_hours) * 100.0)
}

/// Whether an observed cycle-time speedup is a genuine productivity gain
/// rather than a false one.
///
/// Genuine only if cycle time improved (`after < before`) **and** the
/// defect rate did not worsen (`after <= before`). A speedup paired with a
/// worsening defect rate is the chapter's named false-gain pattern and
/// returns `false`, as does no speedup at all.
///
/// # Arguments
///
/// * `cycle_time_before_hours` — full cycle time before AI assistance, in
///   hours.
/// * `cycle_time_after_hours` — full cycle time after AI assistance, in
///   hours.
/// * `defect_rate_before_percent` — defect rate before AI assistance (e.g.
///   change failure rate or escaped defect rate), as a percentage.
/// * `defect_rate_after_percent` — defect rate after AI assistance, as a
///   percentage.
///
/// # Returns
///
/// `true` only if cycle time improved and the defect rate did not worsen.
///
/// # Examples
///
/// ```rust
/// use software_engineering::ai_assisted_development::is_genuine_productivity_gain;
///
/// // Faster, and quality held steady: genuine.
/// assert!(is_genuine_productivity_gain(10.0, 8.0, 2.0, 2.0));
/// // Faster, but quality got worse: the chapter's named false gain.
/// assert!(!is_genuine_productivity_gain(10.0, 8.0, 2.0, 5.0));
/// // No speedup at all: not a gain, genuine or otherwise.
/// assert!(!is_genuine_productivity_gain(10.0, 10.0, 2.0, 2.0));
/// ```
#[must_use]
pub fn is_genuine_productivity_gain(
    cycle_time_before_hours: f64,
    cycle_time_after_hours: f64,
    defect_rate_before_percent: f64,
    defect_rate_after_percent: f64,
) -> bool {
    cycle_time_after_hours < cycle_time_before_hours
        && defect_rate_after_percent <= defect_rate_before_percent
}

#[cfg(test)]
mod tests {
    use super::*;

    // "track whether AI-assisted work moves faster through the cycle-time
    // stages."
    #[test]
    fn net_cycle_time_change_computes_percentage_speedup() {
        let change = net_cycle_time_change_percent(10.0, 8.0).unwrap();
        assert!((change - 20.0).abs() < 1e-6);
    }

    #[test]
    fn net_cycle_time_change_reports_a_slowdown_as_negative() {
        let change = net_cycle_time_change_percent(8.0, 10.0).unwrap();
        assert!(change < 0.0);
    }

    #[test]
    fn net_cycle_time_change_is_none_for_zero_before() {
        assert_eq!(net_cycle_time_change_percent(0.0, 1.0), None);
    }

    // "A genuine productivity gain shows faster cycle time with stable or
    // improved quality."
    #[test]
    fn genuine_gain_requires_both_speed_and_stable_quality() {
        assert!(is_genuine_productivity_gain(10.0, 8.0, 2.0, 2.0));
    }

    // "a false gain shows faster cycle time with degrading quality" — the
    // chapter's named false-gain pattern.
    #[test]
    fn speedup_with_worsening_defect_rate_is_a_false_gain() {
        assert!(!is_genuine_productivity_gain(10.0, 8.0, 2.0, 5.0));
    }

    #[test]
    fn no_speedup_is_not_a_gain_at_all() {
        assert!(!is_genuine_productivity_gain(10.0, 10.0, 2.0, 2.0));
    }
}

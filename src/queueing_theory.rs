//! # Queueing Theory
//!
//! Queueing theory is the mathematical study of waiting lines, and much of
//! a delivery pipeline actually is a queue: a pull request waiting for a
//! reviewer, a commit waiting for a CI runner, a ticket waiting to be
//! picked up. Utilization — how busy a shared, capacity-constrained
//! resource is, as a proportion of its available capacity — is the key
//! quantity: wait time does not grow linearly with utilization, it grows
//! sharply as utilization approaches full capacity.
//!
//! ## Formula
//!
//! ```text
//! Utilization       = arrival rate / service rate
//! Queue is stable    when utilization < 1.0
//! ```
//!
//! ## Why it matters
//!
//! A resource running at 95% busy is often waiting many times longer than
//! one running at 80%, not just "a little worse" — wait time grows sharply,
//! not gradually, as utilization approaches capacity. A queue running at
//! 100% utilization on average has effectively infinite wait time in
//! practice, because real arrivals are uneven, not perfectly smooth. This
//! is why "our reviewers are almost always busy" is a warning sign about
//! wait times to come, not evidence of efficient resourcing, and why
//! deliberate headroom below full utilization is a design choice, not
//! waste.
//!
//! ## Example
//!
//! The topic doc's own worked example: a shared CI fleet found running
//! above 90% utilization during core hours — well past the point where
//! queueing theory predicts wait time grows sharply rather than gradually.
//! A queue with an arrival rate of 9 jobs/hour against a service rate of 10
//! jobs/hour runs at 90% utilization and is still stable; one with an
//! arrival rate of 11 jobs/hour against the same service rate is not.
//!
//! ```rust
//! use software_engineering::queueing_theory::{utilization, is_queue_stable};
//!
//! let near_capacity = utilization(9.0, 10.0).unwrap();
//! assert!((near_capacity - 0.9).abs() < 1e-9);
//! assert!(is_queue_stable(near_capacity));
//!
//! let overloaded = utilization(11.0, 10.0).unwrap();
//! assert!(!is_queue_stable(overloaded));
//! ```
//!
//! ## Pitfalls
//!
//! - **Sizing a shared resource's capacity to match its average arrival
//!   rate exactly**: guarantees high utilization and runaway wait times
//!   whenever demand is even briefly uneven. Plan deliberate headroom.
//! - **Reporting only mean wait time, never a percentile**: hides the long
//!   tail that matters most to the people actually waiting in it.
//! - **Blending success, failure, and skip into one throughput number**:
//!   a team under pressure can make throughput look healthy by quietly
//!   letting the skip rate (abandoned or silently dropped work) rise. Track
//!   arrival, success, failure, and skip rate as four separate numbers.
//! - **Treating "our people are always busy" as a compliment**: it is a
//!   symptom of high utilization, the leading cause of long, unpredictable
//!   wait times.
//!
//! ## Sources
//!
//! - Chapter 2.7, "Queueing theory."
//! - Little, John D. C. "A Proof for the Queuing Formula: L = λW." *Operations
//!   Research*, 1961.
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/02-07-queueing-theory.md

/// Utilization: arrival rate divided by service rate for a shared,
/// capacity-constrained resource.
///
/// # Arguments
///
/// * `arrival_rate` — rate at which new work arrives at the resource.
/// * `service_rate` — rate at which the resource can process work.
///
/// # Returns
///
/// `Some(utilization)` as a fraction (e.g. `0.9` for 90% busy), or `None`
/// when `service_rate` is zero.
///
/// # Examples
///
/// ```rust
/// use software_engineering::queueing_theory::utilization;
///
/// let u = utilization(9.0, 10.0).unwrap();
/// assert!((u - 0.9).abs() < 1e-9);
/// assert_eq!(utilization(9.0, 0.0), None);
/// ```
#[must_use]
pub fn utilization(arrival_rate: f64, service_rate: f64) -> Option<f64> {
    if service_rate == 0.0 {
        None
    } else {
        Some(arrival_rate / service_rate)
    }
}

/// Whether a queue is stable: utilization strictly less than 1.0.
///
/// A queue at or above 100% utilization has, in practice, unboundedly
/// growing wait time, because real arrivals are uneven rather than
/// perfectly smooth. Wait time grows sharply, not gradually, as utilization
/// approaches this threshold — treat utilization consistently near 1.0 as
/// an early warning, not just outright instability as the only concern.
///
/// # Arguments
///
/// * `utilization` — the resource's utilization as a fraction (see
///   [`utilization`]).
///
/// # Returns
///
/// `true` if `utilization < 1.0` (stable), `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use software_engineering::queueing_theory::is_queue_stable;
///
/// assert!(is_queue_stable(0.9));
/// assert!(!is_queue_stable(1.0));
/// assert!(!is_queue_stable(1.1));
/// ```
#[must_use]
pub fn is_queue_stable(utilization: f64) -> bool {
    utilization < 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    // "A utilization analysis found the fleet running above 90% busy during
    // core hours, well past the point where queueing theory predicts wait
    // time grows sharply."
    #[test]
    fn utilization_near_ninety_percent_is_still_stable_but_near_capacity() {
        let u = utilization(9.0, 10.0).unwrap();
        assert!((u - 0.9).abs() < 1e-9);
        assert!(is_queue_stable(u));
        assert_eq!(utilization(9.0, 0.0), None);
    }

    // "A queue running at 100% utilization on average has effectively
    // infinite wait time in practice."
    #[test]
    fn a_queue_at_or_above_full_utilization_is_not_stable() {
        assert!(!is_queue_stable(1.0));
        let overloaded = utilization(11.0, 10.0).unwrap();
        assert!(!is_queue_stable(overloaded));
    }
}

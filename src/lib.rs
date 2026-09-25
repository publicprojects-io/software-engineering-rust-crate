//! # Software Engineering
//!
//! Rust implementations of 22 software-engineering delivery, quality, and
//! reliability metrics — one module per topic. Each module implements the
//! formulas from its source chapter in the book *Software Engineering
//! Metrics* (`software-engineering-metrics/locales/en-001/chapters/*.md`),
//! documents them with runnable examples, and reproduces the chapter's
//! worked example (where the chapter gives one) in its unit tests.
//!
//! The crate is `std`-only: no external dependencies, all quantities are
//! `f64` (or a small enum where the source material calls for a category,
//! such as a SPACE dimension or a vulnerability severity), and functions
//! return `Option<f64>` wherever a denominator could be zero.
//!
//! ## Quickstart
//!
//! A speed improvement, paired with its guardrail, funding a return:
//!
//! ```
//! use software_engineering::flow_framework::{littles_law_wip, flow_efficiency_percent};
//! use software_engineering::dora_metrics::change_failure_rate_percent;
//! use software_engineering::return_on_investment::roi;
//!
//! // A team arrives at 4 items/week with a 5-week flow time: Little's law
//! // says ~20 items are in process in the value stream at any moment.
//! let wip = littles_law_wip(4.0, 5.0);
//! assert_eq!(wip, 20.0);
//!
//! // Of those 5 weeks, only 1.5 weeks is active work: 30% flow efficiency.
//! let efficiency = flow_efficiency_percent(1.5, 5.0).unwrap();
//! assert!((efficiency - 30.0).abs() < 1e-9);
//!
//! // A speed improvement must be paired with its stability guardrail:
//! // change failure rate falls from 25% to 8% as the team ships faster,
//! // not despite it — the DORA finding that speed and stability move
//! // together in elite performers.
//! let cfr_after = change_failure_rate_percent(8.0, 100.0).unwrap();
//! assert!(cfr_after < 25.0);
//!
//! // The combined delivery investment returns 2.0 (200%) on its cost —
//! // every $1 invested returns $2 in net profit, a 3x total return.
//! let r = roi(300_000.0, 100_000.0).unwrap();
//! assert_eq!(r, 2.0);
//! ```
//!
//! ## Module index by theme
//!
//! **Flow metrics** —
//! [`flow_framework`], [`cycle_time`], [`queueing_theory`],
//! [`lean_value_stream_metrics`], [`pull_request_metrics`], [`dora_metrics`]
//!
//! **Developer experience** —
//! [`space_framework`], [`developer_experience_metrics`]
//!
//! **Code and quality** —
//! [`code_complexity`], [`test_effectiveness`], [`code_churn`],
//! [`technical_debt`]
//!
//! **Product and business** —
//! [`escaped_defects`], [`feature_adoption`], [`unit_economics`],
//! [`return_on_investment`]
//!
//! **Reliability, operations, and security** —
//! [`error_budget`], [`incident_metrics`], [`on_call_metrics`],
//! [`vulnerability_management`]
//!
//! **AI-assisted development** —
//! [`ai_assisted_development`]
//!
//! **Metrics programs** —
//! [`maturity_model`]

#![deny(clippy::pedantic)]
#![deny(missing_docs)]

pub mod ai_assisted_development;
pub mod code_churn;
pub mod code_complexity;
pub mod cycle_time;
pub mod developer_experience_metrics;
pub mod dora_metrics;
pub mod error_budget;
pub mod escaped_defects;
pub mod feature_adoption;
pub mod flow_framework;
pub mod incident_metrics;
pub mod lean_value_stream_metrics;
pub mod maturity_model;
pub mod on_call_metrics;
pub mod pull_request_metrics;
pub mod queueing_theory;
pub mod return_on_investment;
pub mod space_framework;
pub mod technical_debt;
pub mod test_effectiveness;
pub mod unit_economics;
pub mod vulnerability_management;

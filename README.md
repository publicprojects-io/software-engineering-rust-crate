# Software Engineering Rust crate

Software engineering metrics models, structs, calculations, and examples —
29 modules covering delivery flow, developer experience, code and test
quality, product and business outcomes, reliability and security, and
AI-assisted development. One module per topic, based on the book *Software
Engineering Metrics*. Every SPACE dimension (chapter 3.1) — Satisfaction,
Performance, Activity, Communication, and Efficiency — now has its own
dedicated module.

The crate is `std`-only with **minimal dependencies**: no module's public
API takes or returns a [`rusty-money`](https://crates.io/crates/rusty-money)
type, but a few financial modules' docs show how to combine their plain
`f64` functions with `rusty_money::Money` directly in your own code (see
each module's own "Money" section). All quantities are `f64` (except a few
naturally integer or enum-typed values, such as McCabe cyclomatic
complexity and severity levels), and functions return `Option<f64>`
wherever a denominator could be zero.

## Install

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
software-engineering = "1.0"
```

## Quickstart

A DORA delivery-stability improvement, turned into a financial case:

```rust
use software_engineering::dora_metrics::change_failure_rate_percent;
use software_engineering::return_on_investment::roi;

// A platform team's change failure rate falls from 25% to 8% across
// 100 production deployments/year.
let cfr_before = change_failure_rate_percent(25.0, 100.0).unwrap();
let cfr_after = change_failure_rate_percent(8.0, 100.0).unwrap();
assert_eq!(cfr_before, 25.0);
assert_eq!(cfr_after, 8.0);

// 17 fewer failed deployments/year, each avoiding $12,000 of incident
// cost, is $204,000/year of benefit against a $150,000 investment.
let failed_deployments_avoided = 100.0 * (cfr_before - cfr_after) / 100.0;
assert_eq!(failed_deployments_avoided, 17.0);
let benefit = failed_deployments_avoided * 12_000.0;
assert_eq!(benefit, 204_000.0);

// ROI = (benefit − cost) / cost = 36%.
let return_on_investment = roi(benefit, 150_000.0).unwrap();
assert!((return_on_investment - 0.36).abs() < 1e-9);
```

## Money

`technical_debt`, `unit_economics`, and `return_on_investment` each show,
in their own module docs, how to use
[`rusty-money`](https://crates.io/crates/rusty-money)'s `Money` type
directly alongside their plain-`f64` functions — this crate has no
`Money`-wrapping functions of its own:

```rust
use rusty_money::{Money, iso};
use software_engineering::return_on_investment::roi;

let benefit = Money::from_major(300_000, iso::USD);
let cost = Money::from_major(100_000, iso::USD);

// rusty_money's own sub() catches a currency mismatch before it ever
// reaches roi(), which only ever sees plain, same-unit f64 amounts.
let net_benefit = benefit.sub(cost).unwrap();
assert_eq!(net_benefit, Money::from_major(200_000, iso::USD));

let r = roi(benefit.to_f64_lossy(), cost.to_f64_lossy()).unwrap();
assert!((r - 2.0).abs() < 1e-9);

// A benefit and cost in different currencies is an error, not a silent
// unit mismatch.
let eur_cost = Money::from_major(100_000, iso::EUR);
assert!(benefit.sub(eur_cost).is_err());
```

## Module index by theme

### Flow metrics

- `flow_framework` — flow velocity, flow distribution, flow time, flow load, Little's law, flow efficiency
- `cycle_time` — the five-stage cycle-time breakdown (coding, pickup, review, test, deploy)
- `queueing_theory` — utilization and queue stability
- `lean_value_stream_metrics` — percent complete and accurate, rolled throughput yield, takt time
- `pull_request_metrics` — time to first review, reviewer load concentration
- `dora_metrics` — deployment frequency, lead time for changes, change failure rate, recovery time

### Developer experience

- `space_framework` — the five SPACE dimensions and coverage checks
- `developer_experience_metrics` — focus time, survey response rate
- `satisfaction_metrics` — eNPS-style satisfaction score, satisfaction trend
- `performance_metrics` — converging-signal count, sufficient converging evidence
- `activity_metrics` — commit-splitting gaming signal, activity rate as context only
- `communication_metrics` — cross-team dependency resolution time, time to first contribution

### Code and quality

- `code_complexity` — McCabe cyclomatic complexity
- `test_effectiveness` — test coverage, mutation kill rate
- `code_churn` — code churn, hotspot score
- `technical_debt` — debt carrying cost, with a direct [`Money`](https://crates.io/crates/rusty-money) usage example
- `static_analysis_metrics` — findings per KLOC, severity-weighted finding score
- `documentation_and_knowledge_metrics` — bus factor, documentation coverage

### Product and business

- `escaped_defects` — escaped defect rate
- `feature_adoption` — initial adoption, retained adoption
- `unit_economics` — unit cost, with a direct [`Money`](https://crates.io/crates/rusty-money) usage example
- `return_on_investment` — ROI and ROI as a range, with a direct [`Money`](https://crates.io/crates/rusty-money) usage example
- `customer_and_business_outcome_metrics` — net revenue retention, honestly-scoped outcome claims

### Reliability, operations, and security

- `error_budget` — error budget and burn rate
- `incident_metrics` — MTTD, MTTA, MTTR, mean duration
- `on_call_metrics` — on-call paging concentration
- `vulnerability_management` — severity levels, vulnerability time-to-remediate

### AI-assisted development

- `ai_assisted_development` — net cycle-time change, genuine vs false productivity gain

### Metrics programs

- `maturity_model` — minimum and average maturity scoring across dimensions

## Testing

Every module reproduces its topic's worked example, or a value grounded
directly in the chapter's own text, in unit tests, and every doc example
compiles and asserts under `cargo test --doc`:

```sh
cargo test
```

## Citation

See [`CITATION.cff`](CITATION.cff) for citation metadata.

## License

Any of MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only, at
your option — or contact us for custom license options. See
[`LICENSE.md`](LICENSE.md).

## Tracking

- Package: [software-engineering](https://crates.io/crates/software-engineering)
- Repository: [github.com/joelparkerhenderson/software-engineering-rust-crate](https://github.com/joelparkerhenderson/software-engineering-rust-crate)
- Author: [Joel Parker Henderson](https://joelparkerhenderson.com) — joel@joelparkerhenderson.com

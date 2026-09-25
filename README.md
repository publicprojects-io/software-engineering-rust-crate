# Software Engineering Rust crate

Software engineering metrics models, structs, calculations, and examples —
22 modules covering delivery flow, developer experience, code and test
quality, product and business outcomes, reliability and security, and
AI-assisted development. One module per topic, based on the book *Software
Engineering Metrics*.

The crate is `std`-only with **zero external dependencies**. All quantities
are `f64` (except a few naturally integer or enum-typed values, such as
McCabe cyclomatic complexity and severity levels), and functions return
`Option<f64>` wherever a denominator could be zero.

## Install

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
software-engineering = "0.1"
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

### Code and quality

- `code_complexity` — McCabe cyclomatic complexity
- `test_effectiveness` — test coverage, mutation kill rate
- `code_churn` — code churn, hotspot score
- `technical_debt` — debt carrying cost

### Product and business

- `escaped_defects` — escaped defect rate
- `feature_adoption` — initial adoption, retained adoption
- `unit_economics` — unit cost
- `return_on_investment` — ROI and ROI as a range

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

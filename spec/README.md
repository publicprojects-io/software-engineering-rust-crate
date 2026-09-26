# Specifications

Each file here is the single-source-of-truth contract for one `src/*.rs`
module: its formula, its public function/enum signatures, and the
source chapter it implements. `src/<name>.rs`'s `## Formula` doc-comment
section must match `spec/<name>.md`'s `## Formula` section; if you
change one, change the other in the same commit.

These files are generated from each module's own rustdoc (`//!` module
doc and `///` item docs) — they are a structured *view* of the
implementation's contract, not hand-maintained prose. If a module's
formula, function signatures, or sources change, regenerate the
corresponding spec file from the updated `src/*.rs` rather than editing
`spec/*.md` by hand and letting it drift.

A few modules (`technical_debt`, `return_on_investment`, `unit_economics`)
additionally expose a currency-checked variant of one or more functions,
built on [`rusty-money`](https://crates.io/crates/rusty-money)'s `Money`
type instead of plain `f64` — see each one's own "Money" section below.
These return `Result<_, rusty_money::MoneyError>` rather than `Option`,
since currency mismatch and overflow are real failure modes plain `f64`
doesn't have.

## Flow metrics

- [`flow_framework`](flow_framework.md) — flow velocity, flow distribution, flow time, flow load, Little's law, flow efficiency
- [`cycle_time`](cycle_time.md) — the five-stage cycle-time breakdown (coding, pickup, review, test, deploy)
- [`queueing_theory`](queueing_theory.md) — utilization and queue stability
- [`lean_value_stream_metrics`](lean_value_stream_metrics.md) — percent complete and accurate, rolled throughput yield, takt time
- [`pull_request_metrics`](pull_request_metrics.md) — time to first review, reviewer load concentration
- [`dora_metrics`](dora_metrics.md) — deployment frequency, lead time for changes, change failure rate, recovery time

## Developer experience

- [`space_framework`](space_framework.md) — the five SPACE dimensions and coverage checks
- [`developer_experience_metrics`](developer_experience_metrics.md) — focus time, survey response rate
- [`satisfaction_metrics`](satisfaction_metrics.md) — eNPS-style satisfaction score, satisfaction trend
- [`activity_metrics`](activity_metrics.md) — commit-splitting gaming signal, activity rate as context only

## Code and quality

- [`code_complexity`](code_complexity.md) — McCabe cyclomatic complexity
- [`test_effectiveness`](test_effectiveness.md) — test coverage, mutation kill rate
- [`code_churn`](code_churn.md) — code churn, hotspot score
- [`technical_debt`](technical_debt.md) — debt carrying cost (plain and Money-typed)
- [`static_analysis_metrics`](static_analysis_metrics.md) — findings per KLOC, severity-weighted finding score
- [`documentation_and_knowledge_metrics`](documentation_and_knowledge_metrics.md) — bus factor, documentation coverage

## Product and business

- [`escaped_defects`](escaped_defects.md) — escaped defect rate, severity-weighted score
- [`feature_adoption`](feature_adoption.md) — initial adoption, retained adoption
- [`unit_economics`](unit_economics.md) — unit cost, cost-component split (plain and Money-typed)
- [`return_on_investment`](return_on_investment.md) — ROI and ROI as a range (plain and Money-typed)
- [`customer_and_business_outcome_metrics`](customer_and_business_outcome_metrics.md) — net revenue retention, honestly-scoped outcome claims

## Reliability, operations, and security

- [`error_budget`](error_budget.md) — error budget and burn rate
- [`incident_metrics`](incident_metrics.md) — MTTD, MTTA, MTTR, mean duration
- [`on_call_metrics`](on_call_metrics.md) — on-call paging concentration
- [`vulnerability_management`](vulnerability_management.md) — severity levels, vulnerability time-to-remediate

## AI-assisted development

- [`ai_assisted_development`](ai_assisted_development.md) — net cycle-time change, genuine vs false productivity gain

## Metrics programs

- [`maturity_model`](maturity_model.md) — minimum and average maturity scoring across dimensions

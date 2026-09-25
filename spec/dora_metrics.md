# `dora_metrics` — The DORA Metrics Framework

Rust module: [`src/dora_metrics.rs`](../src/dora_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/02-10-the-dora-metrics-framework.md`

## Formula

```text
Deployment frequency         = deployments / days
Lead time for changes        = deploy time − first commit time
Change failure rate (%)      = (failed deployments / total deployments) × 100
Failed deployment recovery   = restored time − detected time (never the deploy event)
```

## Public API

### `change_failure_rate_percent`

```rust
pub fn change_failure_rate_percent(failed_deployments: f64, total_deployments: f64) -> Option<f64>
```

Change failure rate: the percentage of deployments that caused a failure

### `deployment_frequency_per_day`

```rust
pub fn deployment_frequency_per_day(deployments: f64, days: f64) -> Option<f64>
```

Deployment frequency: how often a team successfully releases to

### `lead_time_for_changes_hours`

```rust
pub fn lead_time_for_changes_hours(first_commit_time_hours: f64, deploy_time_hours: f64) -> f64
```

Lead time for changes: the time from a code change's first commit to its

### `failed_deployment_recovery_time_hours`

```rust
pub fn failed_deployment_recovery_time_hours(detected_time_hours: f64, restored_time_hours: f64) -> f64
```

Failed deployment recovery time (often shortened to MTTR): how long it

## Sources

- Chapter 2.10, The DORA metrics framework.
- Forsgren, Nicole, Jez Humble, and Gene Kim, *Accelerate: The Science of
  Lean Software and DevOps* (2018).

Topic doc: software-engineering-metrics/locales/en-001/chapters/02-10-the-dora-metrics-framework.md

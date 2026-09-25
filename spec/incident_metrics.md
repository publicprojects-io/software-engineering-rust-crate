# `incident_metrics` — Incident Metrics

Rust module: [`src/incident_metrics.rs`](../src/incident_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/06-02-incident-metrics.md`

## Formula

```text
MTTD          = mean(detection durations)
MTTA          = mean(acknowledgement durations)
MTTR          = mean(resolution durations)
Mean duration = mean(total incident durations)
```

## Public API

### `mean_time_to_detect_minutes`

```rust
pub fn mean_time_to_detect_minutes(detection_durations_minutes: &[f64]) -> Option<f64>
```

Mean time to detect (MTTD): the mean, across incidents, of the duration

### `mean_time_to_acknowledge_minutes`

```rust
pub fn mean_time_to_acknowledge_minutes(acknowledgement_durations_minutes: &[f64]) -> Option<f64>
```

Mean time to acknowledge (MTTA): the mean, across incidents, of the

### `mean_time_to_resolve_minutes`

```rust
pub fn mean_time_to_resolve_minutes(resolution_durations_minutes: &[f64]) -> Option<f64>
```

Mean time to resolve or recover (MTTR): the mean, across incidents, of

### `mean_incident_duration_minutes`

```rust
pub fn mean_incident_duration_minutes(total_durations_minutes: &[f64]) -> Option<f64>
```

Mean total incident duration, from detection start to full resolution —

## Sources

- Chapter 6.2, Incident metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/06-02-incident-metrics.md

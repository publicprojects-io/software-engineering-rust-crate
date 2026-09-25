# `developer_experience_metrics` — Developer Experience Metrics

Rust module: [`src/developer_experience_metrics.rs`](../src/developer_experience_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/03-06-efficiency-and-flow.md`

## Formula

```text
Focus block   = a calendar block >= 2.0 hours, uninterrupted
Response rate = (survey responses received / survey invitations sent) x 100%
```

## Public API

### `is_focus_block`

```rust
pub fn is_focus_block(duration_hours: f64) -> bool
```

Whether a calendar block qualifies as protected "focus time".

### `response_rate_percent`

```rust
pub fn response_rate_percent(responses_received: f64, invitations_sent: f64) -> Option<f64>
```

Survey response rate as a percentage: responses received / invitations

## Sources

- Chapter 3.6, Efficiency and flow: deep work and interruptions.
- Chapter 3.7, Developer experience surveys and `DevEx` metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/03-06-efficiency-and-flow.md
Topic doc: software-engineering-metrics/locales/en-001/chapters/03-07-developer-experience-surveys-and-devex-metrics.md

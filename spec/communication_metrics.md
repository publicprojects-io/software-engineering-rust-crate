# `communication_metrics` — Communication and Collaboration Metrics

Rust module: [`src/communication_metrics.rs`](../src/communication_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/03-05-communication-and-collaboration-metrics.md`

## Formula

```text
Cross-team dependency resolution time = resolved at − raised at
    (a request, API change, or coordinated release, team-to-team)

Time to first contribution = first contribution at − joined at
    (onboarding proxy for how well shared understanding flows)
```

## Public API

### `cross_team_dependency_resolution_time_days`

```rust
pub fn cross_team_dependency_resolution_time_days(raised_at_days: f64, resolved_at_days: f64) -> f64
```

Time from a cross-team dependency being raised to being resolved: a

### `time_to_first_contribution_days`

```rust
pub fn time_to_first_contribution_days(joined_at_days: f64, first_contribution_at_days: f64) -> f64
```

Time from a new team member joining to their first meaningful,

## Sources

- Chapter 3.5, Communication and collaboration metrics.
- Forsgren, Storey, Maddila, Zimmermann, Houck, and Butler, "The SPACE of
  Developer Productivity," *ACM Queue* (2021).

Topic doc: software-engineering-metrics/locales/en-001/chapters/03-05-communication-and-collaboration-metrics.md

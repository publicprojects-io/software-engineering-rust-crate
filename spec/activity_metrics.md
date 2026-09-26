# `activity_metrics` — Activity Metrics and Their Limits

Rust module: [`src/activity_metrics.rs`](../src/activity_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/03-04-activity-metrics-and-their-limits.md`

## Formula

```text
Commit substitution gaming signal =
    commit_count rose AND average_commit_size shrank

Activity rate = commits / engineers / weeks
    (a contextual signal only, never a standalone productivity proxy)
```

## Public API

### `is_commit_substitution_gaming_signal`

```rust
pub fn is_commit_substitution_gaming_signal(
    commit_count_before: f64,
    commit_count_after: f64,
    average_commit_size_before: f64,
    average_commit_size_after: f64,
) -> bool
```

Whether a change in commit count and average commit size matches the

### `commits_per_engineer_per_week`

```rust
pub fn commits_per_engineer_per_week(commits: f64, engineers: f64, weeks: f64) -> Option<f64>
```

Commits per engineer per week — a simple activity rate, provided only

## Sources

- Chapter 3.4, Activity metrics and their limits.

Topic doc: software-engineering-metrics/locales/en-001/chapters/03-04-activity-metrics-and-their-limits.md

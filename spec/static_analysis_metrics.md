# `static_analysis_metrics` — Static Analysis and Code Smell Metrics

Rust module: [`src/static_analysis_metrics.rs`](../src/static_analysis_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/04-04-static-analysis-and-code-smell-metrics.md`

## Formula

```text
Findings per KLOC = findings / (lines_of_code / 1000)

Severity-weighted score = Σ (count × severity_weight)
    Critical = 5, Major = 3, Minor = 1
```

## Public API

### `FindingSeverity`

```rust
enum FindingSeverity
```

Severity classification for a static-analysis finding.

### `severity_weight`

```rust
pub fn severity_weight(severity: FindingSeverity) -> f64
```

The fixed weight applied to a finding of a given severity in

### `findings_per_kloc`

```rust
pub fn findings_per_kloc(findings: f64, lines_of_code: f64) -> Option<f64>
```

Findings per thousand lines of code (KLOC) — a normalized density

### `severity_weighted_finding_score`

```rust
pub fn severity_weighted_finding_score(counts: &[(FindingSeverity, f64)]) -> f64
```

A severity-weighted static-analysis finding score, so a spike in

## Sources

- Chapter 4.4, Static analysis and code smell metrics.

Topic doc: software-engineering-metrics/locales/en-001/chapters/04-04-static-analysis-and-code-smell-metrics.md

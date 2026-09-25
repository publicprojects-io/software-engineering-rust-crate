# `escaped_defects` — Escaped Defect Rate and Quality Escapes

Rust module: [`src/escaped_defects.rs`](../src/escaped_defects.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/05-01-escaped-defect-rate-and-quality-escapes.md`

## Formula

```text
Escaped defect rate (%) = escaped defects / (escaped defects + caught defects) × 100
Severity-weighted score  = critical × 5 + major × 3 + minor × 1
```

## Public API

### `escaped_defect_rate_percent`

```rust
pub fn escaped_defect_rate_percent(escaped_defects: f64, caught_defects: f64) -> Option<f64>
```

Escaped defect rate: the percentage of all found defects that escaped to

### `severity_weighted_escaped_defect_score`

```rust
pub fn severity_weighted_escaped_defect_score(critical: f64, major: f64, minor: f64) -> f64
```

A severity-weighted escaped-defect score, so a spike in minor issues

## Sources

- Chapter 5.1, Escaped defect rate and quality escapes.

Topic doc: software-engineering-metrics/locales/en-001/chapters/05-01-escaped-defect-rate-and-quality-escapes.md

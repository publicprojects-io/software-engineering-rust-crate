# `code_churn` — Code Churn and Hotspot Analysis

Rust module: [`src/code_churn.rs`](../src/code_churn.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/04-03-code-churn-and-hotspot-analysis.md`

## Formula

```text
Code churn    = lines added + lines modified + lines deleted
Hotspot score = churn × complexity

churn      = change volume for a file over a window (commonly 6-12 months)
complexity = a static-complexity measure for the same file (chapter 4.1)
```

## Public API

### `code_churn`

```rust
pub fn code_churn(lines_added: f64, lines_modified: f64, lines_deleted: f64) -> f64
```

Code churn: lines added + lines modified + lines deleted over a window.

### `hotspot_score`

```rust
pub fn hotspot_score(churn: f64, complexity: f64) -> f64
```

Hotspot score: churn × complexity, the combined ranking signal.

## Sources

- Chapter 4.3, Code churn and hotspot analysis.
- Tornhill, Adam, *Your Code as a Crime Scene*.
- Nagappan, Nachiappan, and Thomas Ball, "Use of Relative Code Churn
  Measures to Predict System Defect Density," *ICSE* (2005).

Topic doc: software-engineering-metrics/locales/en-001/chapters/04-03-code-churn-and-hotspot-analysis.md

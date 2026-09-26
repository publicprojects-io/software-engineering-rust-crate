# `documentation_and_knowledge_metrics` — Documentation and Knowledge Metrics

Rust module: [`src/documentation_and_knowledge_metrics.rs`](../src/documentation_and_knowledge_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/04-06-documentation-and-knowledge-metrics.md`

## Formula

```text
Bus factor = the minimum number of people whose combined knowledge
             share meets or exceeds a critical threshold (commonly 50%)

At risk  when bus_factor <= minimum_safe_bus_factor
```

## Public API

### `bus_factor`

```rust
pub fn bus_factor(knowledge_shares_percent: &[f64], critical_threshold_percent: f64) -> Option<usize>
```

The bus factor: the minimum number of people whose combined knowledge

### `is_bus_factor_at_risk`

```rust
pub fn is_bus_factor_at_risk(bus_factor: usize, minimum_safe_bus_factor: usize) -> bool
```

Whether a bus factor is at or below a defined minimum-safe threshold

## Sources

- Chapter 4.6, Documentation and knowledge metrics.
- The "bus factor" (or "truck factor") is a widely used, informally
  named industry concept for knowledge-concentration risk.

Topic doc: software-engineering-metrics/locales/en-001/chapters/04-06-documentation-and-knowledge-metrics.md

# `performance_metrics` — Performance Metrics and Outcome Proxies

Rust module: [`src/performance_metrics.rs`](../src/performance_metrics.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/03-03-performance-metrics-and-outcome-proxies.md`

## Formula

```text
Converging signal count      = count of independent signals indicating
                                positive performance (change failure
                                rate, defect-escape rate, adoption,
                                qualitative peer assessment, ...)
Sufficient converging evidence = converging signal count >= 2
    (no single signal is reliable alone)
```

## Public API

### `converging_signal_count`

```rust
pub fn converging_signal_count(signals: &[bool]) -> usize
```

The number of independent signals, out of those checked, that indicate

### `has_sufficient_converging_evidence`

```rust
pub fn has_sufficient_converging_evidence(signals: &[bool]) -> bool
```

Whether enough independent signals converge to trust a performance

## Sources

- Chapter 3.3, Performance metrics and outcome proxies.
- Forsgren, Storey, Maddila, Zimmermann, Houck, and Butler, "The SPACE of
  Developer Productivity," *ACM Queue* (2021).

Topic doc: software-engineering-metrics/locales/en-001/chapters/03-03-performance-metrics-and-outcome-proxies.md

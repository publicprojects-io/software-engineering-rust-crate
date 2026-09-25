# `maturity_model` — Maturity Model for Engineering Metrics Programs

Rust module: [`src/maturity_model.rs`](../src/maturity_model.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/08-04-maturity-model-for-engineering-metrics-programs.md`

## Formula

```text
Honest overall score = minimum(governance, instrumentation, outcome balance,
                                cultural trust, continuous improvement)
(Average is computed alongside it only to make the gap visible.)
```

## Public API

### `MaturityDimension`

```rust
enum MaturityDimension
```

One of the five maturity-model dimensions, in the chapter's own order.

### `minimum_maturity_level`

```rust
pub fn minimum_maturity_level(scores: &[u8]) -> Option<u8>
```

The minimum score across a set of per-dimension maturity levels — the

### `average_maturity_level`

```rust
pub fn average_maturity_level(scores: &[u8]) -> Option<f64>
```

The arithmetic mean across a set of per-dimension maturity levels — the

## Sources

- Chapter 8.4, Maturity model for engineering metrics programs.
- *Capability Maturity Model Integration (CMMI)*, Software Engineering
  Institute (structural inspiration).

Topic doc: software-engineering-metrics/locales/en-001/chapters/08-04-maturity-model-for-engineering-metrics-programs.md

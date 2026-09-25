# `space_framework` — The SPACE Framework

Rust module: [`src/space_framework.rs`](../src/space_framework.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/03-01-the-space-framework.md`

## Formula

```text
SPACE dimensions (five, not a computed ratio):
  S = Satisfaction and well-being
  P = Performance
  A = Activity
  C = Communication and collaboration
  E = Efficiency and flow

Minimum balanced composition:
  at least one metric from at least 3 of the 5 dimensions,
  mixing objective instrumentation with subjective survey data
```

## Public API

### `SpaceDimension`

```rust
enum SpaceDimension
```

One of the five SPACE dimensions.

### `covers_all_dimensions`

```rust
pub fn covers_all_dimensions(measured: &[SpaceDimension]) -> bool
```

Whether a measured set of dimensions covers all five SPACE dimensions.

### `missing_dimensions`

```rust
pub fn missing_dimensions(measured: &[SpaceDimension]) -> Vec<SpaceDimension>
```

The SPACE dimensions not present in a measured set, in canonical order.

## Sources

- Chapter 3.1, The SPACE framework.
- Forsgren, Storey, Maddila, Zimmermann, Houck, and Butler, "The SPACE of
  Developer Productivity," *ACM Queue* (2021).

Topic doc: software-engineering-metrics/locales/en-001/chapters/03-01-the-space-framework.md

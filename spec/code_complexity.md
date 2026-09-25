# `code_complexity` — Code Complexity Metrics

Rust module: [`src/code_complexity.rs`](../src/code_complexity.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/04-01-code-complexity-metrics.md`

## Formula

```text
Cyclomatic complexity = edges - nodes + 2

edges = control-flow graph edges
nodes = control-flow graph nodes
```

## Public API

### `cyclomatic_complexity`

```rust
pub fn cyclomatic_complexity(edges: i64, nodes: i64) -> i64
```

`McCabe` cyclomatic complexity: independent paths through control flow.

## Sources

- Chapter 4.1, Code complexity metrics.
- `McCabe`, Thomas J., "A Complexity Measure," *IEEE Transactions on
  Software Engineering* (1976).

Topic doc: software-engineering-metrics/locales/en-001/chapters/04-01-code-complexity-metrics.md

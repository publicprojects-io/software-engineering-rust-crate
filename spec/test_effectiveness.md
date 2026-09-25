# `test_effectiveness` — Test Coverage and Test Effectiveness

Rust module: [`src/test_effectiveness.rs`](../src/test_effectiveness.rs)

Source chapter: `software-engineering-metrics/locales/en-001/chapters/04-02-test-coverage-and-test-effectiveness.md`

## Formula

```text
Test coverage %     = lines (or branches) covered / total lines × 100
Mutation kill rate % = mutants killed / mutants total × 100

covered / killed = execution or detection count
total            = denominator (lines, branches, or mutants); zero is undefined
```

## Public API

### `test_coverage_percent`

```rust
pub fn test_coverage_percent(covered: f64, total: f64) -> Option<f64>
```

Test coverage as a percentage: covered lines (or branches) / total × 100.

### `mutation_kill_rate_percent`

```rust
pub fn mutation_kill_rate_percent(mutants_killed: f64, mutants_total: f64) -> Option<f64>
```

Mutation kill rate as a percentage: mutants killed / mutants total × 100.

## Sources

- Chapter 4.2, Test coverage and test effectiveness.
- Jia, Yue, and Mark Harman, "An Analysis and Survey of the Development
  of Mutation Testing," *IEEE Transactions on Software Engineering*
  (2011).

Topic doc: software-engineering-metrics/locales/en-001/chapters/04-02-test-coverage-and-test-effectiveness.md

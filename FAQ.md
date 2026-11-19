# Frequently asked questions

## Why does the API use `f64`?

The source metrics combine durations, rates, percentages, and estimates, and
their chapters express them as ordinary arithmetic. `f64` keeps those formulas
small and composable. Callers with stricter domain constraints should validate
inputs before calling the crate.

## How should I handle money?

Use `rusty_money::Money` (or another checked money type) to add, subtract, and
validate currency amounts, then pass same-unit values to the formula. The
`technical_debt`, `unit_economics`, and `return_on_investment` rustdocs contain
runnable examples.

## Why do some functions return `Option`?

Every division that can have a zero denominator returns `None` for that case.
Use pattern matching or `?` in application code. Only use `unwrap` when a
non-zero denominator has already been established by a nearby invariant.

## Are negative, infinite, or NaN inputs rejected?

No. The crate implements formulas rather than a business-validation policy.
Validate domain rules—such as non-negative counts—at the boundary of your
application. Rust's normal `f64` propagation rules apply.

## Which metric should I start with?

Start from the decision being made, then select multiple signals:

- delivery speed and stability: DORA plus flow/cycle-time metrics;
- developer experience: at least three SPACE dimensions, mixing survey and
  instrumentation data;
- quality: escaped defects plus test effectiveness and static analysis;
- investment decisions: an outcome metric plus unit economics or ROI;
- operations: error budgets plus incident and on-call metrics.

No activity count should be used as an individual productivity score.

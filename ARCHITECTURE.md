# Architecture

This is a `std`-only library of independent, pure metric calculations. Each
topic has one `src/<topic>.rs` implementation and one generated
`spec/<topic>.md` contract. The spec is authoritative.

## Decisions

- **Independent modules:** modules do not import one another. Callers compose
  results explicitly, keeping coupling and feature interactions at zero.
- **Plain inputs:** durations, counts, ratios, and money amounts enter as plain
  numbers. The library performs no I/O, clock access, or network access.
- **Defined failure:** a zero denominator returns `None`; formulas without an
  undefined case return a value directly. Public functions do not panic for
  ordinary inputs.
- **Money at the boundary:** financial formulas accept same-unit `f64` values.
  Callers needing currency and overflow checks use `rusty_money::Money` before
  conversion, as shown in the module documentation.
- **Inline verification:** unit tests live next to the formula they verify;
  runnable rustdoc examples verify public usage.
- **Small dependency surface:** `rusty-money` is the sole runtime dependency.

## Change flow

```text
source chapter -> module rustdoc/API -> generated spec contract
                           |-> inline unit tests
                           |-> README and lib.rs module indexes
```

See `AGENTS.md` for the mandatory module documentation template and validation
commands.

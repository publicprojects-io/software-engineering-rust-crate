# AGENTS.md

Guidance for AI coding agents (and humans) working in this repository.

## What this crate is

`software-engineering` is a `std`-only Rust library, with minimal
dependencies, of software-engineering metrics: one module per topic (flow,
DORA, DevEx, code quality, cost, reliability, security, AI-assisted
development, and metrics-program maturity), each implementing the
formulas and worked examples from one chapter of the sibling book
*Software Engineering Metrics* (see [`spec/`](spec/) and "Adding a new
module" below). Its one real dependency is
[`rusty-money`](https://crates.io/crates/rusty-money), which a handful of
financial modules' docs show how to use directly alongside this crate's
own plain-`f64` functions (see "Money convention" below) — no module's
public API takes or returns a `rusty_money` type itself.

## Commands

```sh
cargo build                          # compile the library
cargo test                           # run unit tests + doctests (both must pass)
cargo test --lib                     # unit tests only
cargo test --doc                     # doctests only
cargo test <module>::                # run one module's tests, e.g. cargo test dora_metrics::
cargo test <module>::tests::<name>   # run a single test by name
cargo clippy --all-targets           # lint — see "Lint discipline" below, this must be clean
cargo doc --no-deps --open           # build and view rustdoc locally
```

There is no `tests/` directory — all tests live inline in each
`src/*.rs` file under `#[cfg(test)] mod tests`, next to the code they
exercise.

## Architecture

- One file per topic in `src/`, each declared with `pub mod` in
  `src/lib.rs`. Modules are independent of each other (no cross-module
  imports) — each is a self-contained set of pure functions over `f64`
  (or a small `Copy` enum where the source material calls for a
  category, e.g. a SPACE dimension or a vulnerability severity).
- Every function returns `Option<f64>` (or `Option<T>`) wherever a
  denominator could be zero, returning `None` in that case rather than
  dividing by zero or panicking. Functions that can't have an undefined
  case (e.g. a plain subtraction like `flow_time`) return a bare value.
  This applies to every function in every module — none take or return a
  `rusty_money` type; see "Money convention" below for how those two
  worlds combine in a caller's own code instead.
- No module depends on I/O or time — all inputs (durations, counts,
  percentages) are passed in by the caller as plain numbers; the crate
  has no concept of "now" or "the network."
- `src/lib.rs`'s module doc comment is the crate's own module index by
  theme, mirrored in `README.md`'s "Module index by theme" section —
  keep both in sync when adding or re-categorizing a module.

## Module doc structure (every `src/*.rs` file follows this exactly)

```rust
//! # Title Case Name
//!
//! Prose introduction: what the metric is, why it exists.
//!
//! ## Formula
//!
//! ```text
//! plain-text formula(s), synthesized from the source chapter's prose
//! ```
//!
//! ## Why it matters
//!
//! ## Example
//!
//! ```rust
//! // a runnable, narratively-framed doctest
//! ```
//!
//! ## Pitfalls
//!
//! - bullet list, drawn from the source chapter's own anti-patterns
//!
//! ## Sources
//!
//! - Chapter X.Y, Chapter title.
//! - any bibliography entries the chapter itself cites
//!
//! Topic doc: software-engineering-metrics/locales/en-001/chapters/<NN>-<NN>-<slug>.md
```

Every public function then gets a full `///` doc comment: one-line
summary, an explanation paragraph, `# Arguments`, `# Returns`, and a
runnable `# Examples` doctest. `#[cfg(test)] mod tests` sits at the
bottom of the file; each test is preceded by a `//` comment quoting or
paraphrasing the specific line of the source chapter it's verifying.

## Lint discipline

`src/lib.rs` carries `#![deny(clippy::pedantic)]` and
`#![deny(missing_docs)]` crate-wide — `cargo build` and
`cargo clippy --all-targets` must both be clean before you're done.
In practice this means:

- Every public function with no side effects needs `#[must_use]`.
- Every public item (module, function, struct, enum, and enum variant)
  needs a doc comment — `missing_docs` is a hard error, not a
  suggestion.
- In **unit tests** (not doctests), never compare a computed `f64` with
  `==`/`assert_eq!` — use `assert!((a - b).abs() < 1e-9)`
  (`clippy::float_cmp` is part of the pedantic group). Exact,
  non-computed literals (e.g. an enum-driven constant) may still use
  `assert_eq!`. Doctests are not scanned by clippy, so a doctest may use
  direct `assert_eq!` on an exact result — this crate's `README.md` and
  `src/lib.rs` quickstart examples do exactly that.
- A necessary `usize as f64` cast (e.g. dividing a sum by a count) needs
  a one-line comment justifying why precision loss can't matter here,
  plus `#[allow(clippy::cast_precision_loss)]` on that one line — see
  `src/pull_request_metrics.rs` or `src/maturity_model.rs` for the
  pattern.

## Money convention

`technical_debt`, `return_on_investment`, and `unit_economics` each have a
"Money" section in their module doc showing how to combine that module's
plain-`f64` function(s) with
[`rusty_money::Money`](https://docs.rs/rusty-money) directly — this crate
does **not** provide `_money`-suffixed wrapper functions around
`rusty_money`'s own API. Earlier drafts of this crate did add such
wrappers (`debt_carrying_cost_money`, `roi_money`, `unit_cost_money`, and
similar); they were removed because they added no logic beyond one or two
chained `rusty_money` method calls the caller can write just as easily
themselves. When a financial module needs a "Money" section, follow the
same pattern instead of reintroducing a wrapper:

- Write a runnable doctest in the module doc's `## Money` section that
  imports `rusty_money::{Money, iso}`, builds `Money` values with
  `Money::from_major`/`Money::from_minor`, and calls `rusty_money`'s own
  `.add`/`.sub`/`.mul`/`.div` directly — these already return `Result`, so
  a currency mismatch or overflow surfaces as an `Err` with no adapter
  needed.
- Where the module's plain-`f64` function needs a plain number derived
  from `Money` (e.g. `roi`'s `benefit`/`cost` arguments), call
  [`rusty_money::Money::to_f64_lossy`](https://docs.rs/rusty-money) to
  convert, and show that conversion in the same doctest — don't add a
  crate function whose only job is that conversion plus a delegate call.
- `rusty_money::Money::mul`/`Money::div` take `N: Into<Decimal>`, which
  plain `f64` does not implement (deliberately, since an `f64`→`Decimal`
  conversion can be lossy) — use an integer literal (`.mul(12)`, not
  `.mul(12.0)`) in these examples.
- When sorting or comparing `f64` inside a function that could otherwise
  panic on `NaN` (e.g. `partial_cmp().unwrap()`), prefer `f64::total_cmp`
  instead — it never panics, so there's nothing to document, and it
  avoids `clippy::missing_panics_doc` (part of the pedantic group)
  entirely rather than working around it. This isn't Money-specific, but
  came up while working on the Money examples above.

## Adding a new module

1. Find the module's source chapter in the sibling `software-engineering-metrics`
   book repository's `locales/en-001/chapters/` directory (filename
   pattern `<NN>-<NN>-<slug>.md`, matching "Chapter N.M" in the
   module's `## Sources` section). Chapters are prose-only — there is
   no literal formula to copy; synthesize a `Formula` block that's
   faithful to the chapter's recommendations, the way every existing
   module already does.
2. Add `pub mod <name>;` to `src/lib.rs` (alphabetical order) and to its
   themed doc-comment index, and add the module to `README.md`'s
   "Module index by theme" section.
3. Write `src/<name>.rs` following the structure above.
4. Add a `spec/<name>.md` entry (see [`spec/README.md`](spec/README.md))
   — the spec is the single-source-of-truth contract (formula,
   function signatures, zero-guard behavior) that the implementation
   must match; keep it and the module doc's `## Formula` section in
   sync.
5. Run `cargo build`, `cargo clippy --all-targets`, `cargo test`, and
   `cargo doc --no-deps` — all four must be clean before the module is
   considered done.

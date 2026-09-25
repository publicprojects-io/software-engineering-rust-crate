# AGENTS.md

Guidance for AI coding agents (and humans) working in this repository.

## What this crate is

`software-engineering` is a `std`-only, zero-external-dependency Rust
library of software-engineering metrics: one module per topic (flow,
DORA, DevEx, code quality, cost, reliability, security, AI-assisted
development, and metrics-program maturity), each implementing the
formulas and worked examples from one chapter of the sibling book
*Software Engineering Metrics* (see [`spec/`](spec/) and "Adding a new
module" below).

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
- No module depends on I/O, time, or any external crate — all inputs
  (durations, counts, percentages) are passed in by the caller as plain
  numbers; the crate has no concept of "now" or "the network."
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

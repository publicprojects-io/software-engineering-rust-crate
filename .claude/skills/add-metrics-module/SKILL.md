---
name: add-metrics-module
description: Add a new metrics module to this crate from a chapter of the sibling "Software Engineering Metrics" book — synthesizes a formula from prose, writes the module following this crate's exact doc/test/lint conventions, updates spec/, lib.rs, README.md, llms.txt, and llms.json, and validates with build/clippy/test/doc. Use when asked to add a metric, cover a new chapter, or extend this crate's module set.
---

# add-metrics-module

Add one new topic module to the `software-engineering` crate, end to end.
This crate is a `std`-only, zero-dependency Rust library where every
module implements the formulas and worked examples from one chapter of
the sibling `software-engineering-metrics` book. Full conventions live in
[`AGENTS.md`](../../../AGENTS.md) — read it first if this is your first
time in this repo; this skill is the step-by-step checklist for the one
recurring task of adding a module.

## Steps

1. **Find the source chapter.** Locate it in the sibling
   `software-engineering-metrics` repository's
   `locales/en-001/chapters/` directory — filename pattern
   `<NN>-<NN>-<slug>.md`. Read it in full. These chapters are prose-only:
   there is no literal formula to copy. Identify the 1–4 concrete
   quantities the chapter's recommendations imply (a rate, a percentage,
   a duration, a classification) and synthesize a formula faithful to
   the chapter's own worked numbers where it gives any (grep for
   percentages, "roughly N", or a stated example first — reproduce those
   exactly if present, the way `error_budget`'s "roughly 43 minutes"
   worked example does).

2. **Design the public API** before writing prose. For each function,
   decide:
   - Does a real input make its denominator zero? If so, return
     `Option<f64>` and return `None` in that case — never panic, never
     divide by zero silently.
   - Is it a plain, always-defined computation (a subtraction, a sum)?
     Return the bare value.
   - Does the chapter call for a fixed category (a severity scale, a
     named dimension)? Add a small `#[derive(Debug, Clone, Copy,
     PartialEq, Eq, Hash)]` enum, matching the pattern in
     `src/space_framework.rs` or `src/vulnerability_management.rs`.

3. **Write `src/<name>.rs`** following the exact structure documented in
   `AGENTS.md`'s "Module doc structure" section: a module `//!` doc
   comment with `## Formula` / `## Why it matters` / `## Example` /
   `## Pitfalls` / `## Sources`, then every public item with a full
   `///` doc comment (`# Arguments`, `# Returns`, `# Examples` with a
   runnable doctest), then `#[cfg(test)] mod tests` at the bottom with
   each test preceded by a `//` comment quoting the chapter line it
   verifies. Use `src/technical_debt.rs` and `src/space_framework.rs` as
   style templates — copy their shape, not their content.

4. **Satisfy the lint discipline** as you write, not after:
   - `#[must_use]` on every public, side-effect-free function.
   - Every public item documented — `#![deny(missing_docs)]` is
     crate-wide.
   - In unit tests, compare floats with `assert!((a - b).abs() <
     1e-9)`, never `assert_eq!` on a computed value. Doctests are exempt
     (clippy doesn't scan them) and may use direct `assert_eq!` on exact
     literal results.
   - A `usize as f64` cast needs a one-line justification comment plus
     `#[allow(clippy::cast_precision_loss)]` on that line.

5. **Wire it in:**
   - Add `pub mod <name>;` to `src/lib.rs` (alphabetical order) and to
     its themed doc-comment module index.
   - Add the module to `README.md`'s "Module index by theme" section,
     matching the theme grouping already used there.
   - Regenerate its `spec/<name>.md` from the new module's rustdoc
     (extract `## Formula`, `## Sources`, and each public function's
     signature + one-line summary — see `spec/README.md` for the exact
     shape) and add it to `spec/README.md`'s theme list.
   - Add the module to `llms.txt`'s matching theme section, and add its
     entry (name, title, description, chapter, source_chapter_path,
     spec, source, functions) to `llms.json`.

6. **Validate — all four must be clean:**
   ```sh
   cargo build
   cargo clippy --all-targets
   cargo test
   cargo doc --no-deps
   ```
   `cargo test` must show new passing unit tests *and* new passing
   doctests for the module (check the `Doc-tests software_engineering`
   section of the output specifically — a module with doc examples but
   no `cargo test --doc` coverage means a doctest didn't compile as a
   real test).

## Common mistakes this skill exists to prevent

- **Treating the source chapter as containing a formula to transcribe.**
  It doesn't — every chapter in this book is prose recommendations, not
  equations. The formula is your synthesis, held to a standard of being
  faithful to the chapter's own recommendations and worked numbers.
- **Skipping the `spec/`, `llms.txt`, and `llms.json` updates.** These
  three are a single source of truth alongside the code; a module that
  exists only in `src/` and `README.md` has drifted from the rest of the
  documentation set.
- **Using `assert_eq!` on a computed float in a unit test.** This passes
  `cargo test` but fails `cargo clippy --all-targets` under this crate's
  `#![deny(clippy::pedantic)]`.

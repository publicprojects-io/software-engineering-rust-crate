# Roadmap

The crate's implemented scope is the 29 contracts under `spec/`. New metric
modules are added only when a source chapter and a precise public contract are
available. The sibling book, not speculative API growth, determines that
content roadmap.

## Active engineering work

- Keep region, function, and line coverage at 100%, while retaining boundary
  and worked-example assertions rather than optimizing only for the number.
- Automate detection of spec, module-index, and package drift.
- Run advisory, license, MSRV, stable, nightly, and cross-platform checks.
- Maintain mutation and property testing for formula invariants.
- Produce reproducible release artifacts, SBOMs, and checksums.

## Completed audit work

The November 2026 audit drove the changelog, badges, docs.rs metadata, security
policy, learning guides, examples, architecture and dependency documentation,
coverage enforcement, CI expansion, and contributor templates.

## Deliberate non-goals

Some audit suggestions conflict with the source-of-truth architecture or would
add complexity without user evidence:

- cross-module imports and framework objects;
- integration tests in `tests/` (the repository contract requires inline unit
  tests and doctests);
- wrapper APIs around `rusty_money`;
- `f32`/Decimal duplicates, builders, or serialization on APIs that are plain
  functions;
- `#[inline]` annotations without benchmark evidence;
- `no_std` while the crate is explicitly documented as `std`-only;
- code signing for binaries when the package ships no binaries.

Community promotion, third-party submissions, videos, social posts, and
publishing releases require maintainer-controlled accounts and editorial
approval. They are release activities, not changes that can be truthfully
completed in a source checkout.

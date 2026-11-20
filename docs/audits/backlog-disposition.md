# Audit backlog disposition

This ledger reconciles the recommendations in
`docs/audits/mistral-medium-3-5-high/index.md`. “Done” means the repository now
contains a verifiable artifact or gate. “Rejected” means the recommendation
conflicts with the source-of-truth architecture or has no applicable artifact.
“Maintainer action” means it needs an external account or editorial decision
and cannot be truthfully completed by changing this checkout.

## Repository work

| Recommendation group | Status | Evidence |
|---|---|---|
| Changelog, badges, docs.rs, security, funding | Done | `CHANGELOG.md`, README, `Cargo.toml`, `SECURITY.md`, `.github/FUNDING.yml` |
| Roadmap, comparison, benchmarks, coverage, architecture | Done | Corresponding top-level Markdown guides |
| Getting started, examples, dependency graph, FAQ, glossary, API/stability policy | Done | Guides plus three runnable `examples/` |
| Migration guidance | Done | Release entries in `CHANGELOG.md` and stability policy |
| PR, issue, and discussion templates | Done | `.github/*_TEMPLATE*` |
| Toolchain alignment and 2024 edition | Done | Rust 1.98 pin, edition 2024, clean pedantic clippy |
| Audit, dependency updates, deny, vet, machete, geiger | Done | Security workflows, Dependabot, policy files, local audit evidence |
| CodeQL analysis | Done | Scheduled and change-triggered Rust and GitHub Actions scans |
| Alphabetical modules and package dry-run | Done | `bin/check-structure`, `bin/ci`, security workflow |
| Generated spec drift | Done | `bin/check-specs` compares every formula, public item list, and function signature in CI |
| Property, mutation, fuzz, benchmark testing | Done | Proptest invariants, mutation workflow/config, public-API fuzz target, Criterion harness |
| Coverage target and llvm-cov | Done | `bin/test-coverage` enforces 100%; CI gate |
| Cross-platform, MSRV, stable, nightly | Done | CI matrices and compatibility job |
| ARM architecture testing | Done | `macos-latest` is an ARM64 GitHub-hosted runner; CI labels the architecture explicitly |
| Check, docs, release, SBOM automation | Done | CI/security/release workflows |
| Typos, Markdown, shell, YAML, manifests, semver, unused dependencies | Done | Quality workflow and lint configs |
| Editor, pre-commit, VS Code, Codespaces, Just | Done | Root tooling/configuration files |
| Error handling, interconnection, usage patterns | Done | `ARCHITECTURE.md`, `FAQ.md`, `GETTING_STARTED.md`, examples |
| `#[must_use]` and clippy exceptions | Done | Pedantic clippy gate; only three justified precision-loss exceptions |
| Float extremes and consistency | Done | Property tests, full API fuzzing, boundary unit tests |
| Supply-chain audit and release integrity | Done | Dated audit, SBOM, checksum, advisory/license gates |
| Release identity verification | Done | GitHub build-provenance attestation plus documented verification command |
| Workflow and documentation links | Done | Immutable action SHAs, least-privilege checkout, and Lychee link checking |
| Repository security settings | Done | Discussions, dependency alerts/fixes, secret scanning, push protection, and private reporting enabled |

## Rejected or superseded recommendations

| Recommendation | Disposition |
|---|---|
| Integration tests under `tests/` | Rejected: `AGENTS.md` explicitly requires inline tests; cross-module scenarios are runnable examples. |
| Snapshot/golden docs | Superseded by 103 executable doctests and `-D warnings` rustdoc. |
| "Understanding the Math" in every module | Superseded by each module's mandatory Formula, worked Example, Pitfalls, and Sources sections. |
| Decision tree | Superseded by the decision-oriented module index and the "Which metric should I start with?" FAQ. |
| Case studies, worksheets, quizzes, certification | Rejected as original editorial content outside the source chapters; runnable worked examples remain in scope. |
| `cargo-nextest` | Rejected: 141 tests finish in hundredths of a second; another runner adds no useful speed. |
| `cargo-udeps` in addition to machete | Superseded by stable, fast `cargo machete`; nightly-only duplication is unjustified. |
| `cargo-bloat` and symbol analysis | Not applicable to a formula library with no binary artifact; Criterion tracks relevant call performance. |
| `cargo-as`, `cargo-inst`, `cargo-synam` | Rejected: these audit recommendations do not identify maintained tools applicable to a library crate. |
| `serde`, `f32`, Decimal, builders, traits | Rejected: current public values are functions and Copy enums; no user contract requires parallel type systems. |
| `no_std` | Rejected: the crate's explicit contract is `std`-only. |
| blanket `const`/`inline` annotations | Rejected without measured benefit or a source-contract need. |
| code signing for binaries | Not applicable: this library ships no binary artifact. |
| breaking-change section in every module | Superseded by centralized changelog/stability policy; empty repeated sections add noise. |
| examples on every enum variant | Superseded by module-level worked examples and exhaustive variant unit tests. |
| deprecated annotations | Not applicable: there are no deprecated public items. |
| test utility macro | Rejected: local assertions are short and preserve the source-specific commentary required by `AGENTS.md`. |
| stress, round-trip, and golden-master suites | Not applicable to deterministic one-way scalar formulas; property, boundary, mutation, and fuzz tests cover their meaningful failure modes. |
| `cargo-readme` | Rejected: README is a decision-oriented project guide, not generated crate-level rustdoc. |
| `cargo-toml-lint` and `cargo-warn` | Superseded by Cargo parsing, `cargo sort --check`, and crate-wide `-D warnings`/pedantic clippy. |
| `cargo-generate` and `cargo-watch` | Rejected as user-global workflow choices; the repository provides templates, Just, Make, and editor tasks without requiring extra tools. |
| performance regression gate | Superseded by the Criterion harness; unstable shared-runner timing is diagnostic rather than a release contract. |

## Maintainer-controlled external actions

Publishing a version, creating social posts/videos/newsletters, submitting to
third-party lists or forums, issuing crev/vet identity proofs, and creating a
website require identity assertions or editorial approval. The repository
contains release automation and source material that can prepare those actions,
but does not claim they occurred.

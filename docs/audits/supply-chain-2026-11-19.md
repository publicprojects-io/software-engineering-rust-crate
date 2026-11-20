# Supply-chain audit — 2026-11-19

## Scope

This audit covers the Rust dependency graph, source use of unsafe Rust,
repository build and test commands, package contents, CI permissions, release
artifacts, disclosure process, and the inspected GitHub repository security
settings at audit time. It does not certify crates.io account configuration,
branch protection, or secret rotation.

## Findings

| Area | Evidence | Result |
|---|---|---|
| Direct runtime dependencies | `Cargo.toml`, `cargo tree` | One: `rusty-money 0.5.1` |
| Resolved vulnerabilities | `cargo audit --no-fetch` against 1,550 cached advisories | None found |
| Licenses and sources | `cargo deny check bans licenses sources` | Pass |
| Dependency review policy | `cargo vet --locked` | Pass with 104 explicit initial exemptions |
| Unused dependencies | `cargo machete` | None found |
| Unsafe in project source | crate-level `#![forbid(unsafe_code)]` | Compile-time prohibition |
| Locked reproducibility | `Cargo.lock`; CI uses `--locked` for package/release work | Present |
| Publishability | `cargo package --offline --locked --allow-dirty` | Package verified |
| MSRV | `rust-version = "1.98"`; matrix for 1.98 and 1.99 | Declared and automated |
| API compatibility | `cargo semver-checks check-release` against 1.1.0 | 229 checks passed; patch release valid |
| Test integrity | 141 unit/property tests, 103 doctests, 468/468 viable mutants caught | Automated |
| Coverage | `bin/test-coverage` | 100% regions/functions/lines enforced |
| Fuzzing | `fuzz/fuzz_targets/public_api.rs` | Every public function exercised |
| Release artifacts | `.github/workflows/release.yml` | Crate, CycloneDX SBOM, SHA-256 sums, provenance attestation |
| CI permissions | Read-only by default; release-artifact job alone has `contents: write` | Least privilege by job purpose |
| Workflow dependencies | Every action is pinned to a full commit SHA | Immutable, Dependabot-managed inputs |
| Code scanning | CodeQL analyzes Rust and GitHub Actions | Automated on changes and weekly |
| Vulnerability disclosure | `SECURITY.md` and private advisory link | Documented |
| Repository security | GitHub API inspection | Discussions, vulnerability alerts, Dependabot fixes, secret scanning, push protection, and private reporting enabled |

## Dependency policy

`deny.toml` rejects wildcard dependencies and unknown registries/git sources,
checks advisories, and permits only the licenses currently required by the
resolved graph. `supply-chain/config.toml` records explicit `cargo-vet`
exemptions instead of claiming reviews that have not occurred. Dependabot
checks Cargo and GitHub Actions weekly. RustSec, `cargo-deny`, and `cargo-vet`
also run weekly and on changes.

Development-only `proptest` and Criterion dependencies are intentionally
isolated under `[dev-dependencies]`; they are not linked into consumer builds.
The fuzz package is unpublished and excluded from the release package.

## Build and release path

`bin/ci` performs formatting, structural, type, pedantic lint, unit/property,
doctest, and rustdoc checks. Tagged releases must rebuild and test before
producing a `.crate`, CycloneDX JSON SBOM, and SHA-256 checksum file. Publishing
is isolated behind the `crates-io` GitHub environment and its token.

## Residual risks

- Advisory results are time-sensitive; scheduled checks, not this document,
  provide continuing assurance.
- `rusty-money` brings transitive numeric dependencies even though dependency-
  owned types do not appear in this crate's public API.

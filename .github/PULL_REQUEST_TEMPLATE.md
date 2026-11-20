# Pull request

## Summary

Describe the user-visible change and the source chapter or issue it addresses.

## Checklist

- [ ] I read `AGENTS.md` and every file under `spec/`.
- [ ] Formula, API, zero-guard behavior, rustdoc, and spec agree.
- [ ] New public items have complete docs, examples, and `#[must_use]` where applicable.
- [ ] Inline tests quote or paraphrase the source behavior they verify.
- [ ] `src/lib.rs` and README module indexes remain synchronized and alphabetical.
- [ ] `cargo fmt --all -- --check`, `cargo clippy --all-targets`, `cargo test`, and
      `cargo doc --no-deps` pass.
- [ ] `bin/coverage` remains at 100% region, function, and line coverage.
- [ ] `CHANGELOG.md` describes user-visible changes.

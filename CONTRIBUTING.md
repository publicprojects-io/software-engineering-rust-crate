# Contributing to Software Engineering Rust crate

Thanks for helping improve this project.

## Ways to contribute

- **Report a problem:** open an issue describing what is wrong, unclear, or missing.
- **Fix or improve something:** open a pull request. Small fixes can go straight to a pull request; for larger changes, please open an issue first so we can agree on the approach.
- **Add content or examples:** follow the structure and style of the existing files.

## Development

Read `AGENTS.md` and `spec/` first; the spec is the source of truth. Layout: one module per topic in
`src/`, each with a matching `spec/<module>.md`; tests are inline (`#[cfg(test)]`) and in doctests.

```sh
make ci                                    # runs bin/ci: fmt, clippy, tests + doctests, rustdoc
make test-minimum-supported-rust-version   # check and test on the MSRV (1.98) and the next minor
```

Run `make ci` before opening a pull request. Adding a module also means updating `src/lib.rs`, the
README module index, and `spec/`.

## Code of conduct

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contribution is offered under the terms in [LICENSE.md](LICENSE.md).

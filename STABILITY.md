# Stability policy

The crate follows Semantic Versioning. Within a major release, existing public
function signatures, enum variants, and documented behavior are preserved.
Additive APIs may appear in minor releases; fixes and documentation changes may
appear in patch releases. Deprecations precede removals whenever practical.

The minimum supported Rust version (MSRV) is the `package.rust-version` value in
`Cargo.toml`, currently 1.98. Raising the MSRV is announced in the changelog and
is at least a minor-version change. CI tests the MSRV and the following compiler
version in addition to the pinned development toolchain.

Floating-point results follow Rust `f64` semantics. Unless a function documents
otherwise, changes that only alter insignificant rounding are not treated as a
compatibility guarantee; formula definitions and zero-denominator behavior are.

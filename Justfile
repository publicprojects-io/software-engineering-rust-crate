default:
    @just --list

ci:
    bin/ci

test:
    cargo test --all-targets --all-features
    cargo test --doc --all-features

coverage:
    bin/coverage

benchmarks:
    bin/benchmarks

msrv:
    bin/test-minimum-supported-rust-version

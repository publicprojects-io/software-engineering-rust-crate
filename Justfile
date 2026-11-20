default:
    @just --list

ci:
    bin/ci

test:
    cargo test --all-targets --all-features
    cargo test --doc --all-features

coverage:
    bin/test-coverage

benchmarks:
    bin/test-benchmarks

msrv:
    bin/make-ci-minimum-supported-rust-version

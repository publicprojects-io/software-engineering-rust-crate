# Test coverage

The required region, function, and line coverage is **100%** for `src/*.rs`.

Install the coverage runner once, then run the project command:

```sh
cargo install cargo-llvm-cov
bin/test-coverage
```

The command runs every library test with every feature, fails if region,
function, or line coverage drops below 100%, and writes an HTML report to
`target/llvm-cov/html/index.html`.
Documentation examples are independently compiled and executed by
`cargo test --doc --all-features` in `bin/ci`.

Coverage shows which code ran; it does not prove that assertions are useful.
Worked examples, boundary cases, property checks, and mutation testing are used
to assess test effectiveness as separate concerns.

The current mutation baseline catches all 468 viable generated mutants. One
operator replacement is excluded as mathematically equivalent: multiplying or
dividing the minor-severity count by its documented weight of `1.0` produces
the same result for every input.

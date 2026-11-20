# Benchmarks

The crate consists of small, allocation-free formulas except for APIs that
intentionally return a `Vec`. Performance is normally dominated by the caller's
data acquisition, so benchmarks are diagnostic rather than an API promise.

Run all benchmark harnesses with:

```sh
bin/test-benchmarks
```

Record the toolchain, CPU, operating system, power mode, and command with any
published result. Compare results from the same machine and toolchain; timings
from unlike environments are not directly comparable. Generated reports belong
under `target/` and are not committed.

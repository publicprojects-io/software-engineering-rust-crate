# Dependencies

## Module graph

All 29 public modules are peers and have no cross-module imports:

```text
caller
  |-- flow_framework
  |-- dora_metrics
  |-- developer_experience_metrics
  |-- ... 26 other independent modules
```

This lets callers select and compose metrics without activating unrelated
parts of the API.

## Crate graph

`software-engineering` has one direct runtime dependency:

```text
software-engineering -> rusty-money -> rust_decimal
                                   -> num-traits
```

The dependency supports the runnable financial documentation examples. No
public function accepts or returns a dependency-owned type. `Cargo.lock` is the
authoritative resolved graph for this repository; use `cargo tree` to inspect
it and `cargo deny check`/`cargo audit` for policy and advisory checks.

`supply-chain/config.toml` is the `cargo-vet` review ledger. Its initial
third-party exemptions are explicit and version-specific; replacing them with
real audits is preferable to silently broadening or deleting the policy.

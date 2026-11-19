# Comparison and fit

This crate is a formula library, not a telemetry platform.

| Need | This crate | Dashboard/observability product | Custom formulas |
|---|---|---|---|
| Collect data from tools | No | Usually | You build it |
| Auditable, sourced formulas | Yes | Varies | Varies |
| Small Rust dependency | Yes | No | Yes |
| Organization-specific validation | Caller supplies it | Configurable | You build it |
| Hosted charts and storage | No | Usually | You build it |
| Independent pure functions | Yes | Rarely exposed | If designed that way |
| Zero-denominator handling | Consistent `Option` | Varies | You decide |

Choose this crate when a Rust system already has metric observations and needs
small, reviewable calculations tied to published software-engineering guidance.
Choose a telemetry product when collection, persistence, alerting, and charts
are the main need. Custom code remains appropriate for organization-specific
definitions not represented by the source chapters.

The project does not claim API or feature parity with similarly named crates;
evaluate alternatives against their current documentation and maintenance state.

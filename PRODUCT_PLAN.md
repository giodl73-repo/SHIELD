# SHIELD Product Plan

## Thesis

Score care-delivery networks at a declared scale, identify measurable access,
capacity, workforce, continuity, affordability, and resilience gaps, and report
where infrastructure methods do not transfer cleanly.

## Implemented product shape

- Seven-crate Rust workspace: six shared analysis crates plus the feature-specific
  `shield-cms-access` public-data result.
- DIM-01..13 scale-aware evidence contracts.
- Explicit transfer-strain and null-result posture.
- A deterministic CMS-derived aggregate fixture; no patient records.
- A fourteen-section held HLT handoff that cannot authorize savings, allocation,
  facility or clinical action, public release, or rate changes.

## Next public work

The first public aggregate-data slice is complete. Next:

1. Join a bounded official rurality or shortage-area geography without inferring
   access from state totals.
2. Add staffed service-line capacity only where a compatible public denominator
   and baseline/surge basis exist.
3. Test a specific HLT candidate against access, quality/safety, equity,
   adequacy/resilience, cost, and delivery evidence before any fiscal admission.

## Non-goals

No medical advice, patient-level analysis, licensing, accreditation,
Certificate-of-Need, payer, coverage, or treatment determination.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p shield-cli -- --help
```

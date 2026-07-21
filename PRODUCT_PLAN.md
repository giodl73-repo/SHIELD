# SHIELD Product Plan

## Thesis

Score care-delivery networks at a declared scale, identify measurable access,
capacity, workforce, continuity, affordability, and resilience gaps, and report
where infrastructure methods do not transfer cleanly.

## Implemented product shape

- Six-crate Rust workspace for network, corpus, score, tier, gap, and CLI.
- DIM-01..13 scale-aware evidence contracts.
- Explicit transfer-strain and null-result posture.
- Deterministic synthetic fixtures; no patient records.

## Next public work

1. Select a bounded public aggregate-data corpus.
2. Publish source, privacy, and interpretation constraints before analysis.
3. Separate measurable access/capacity from socially constructed outcomes.
4. Review the first finding through clinical, payer, workforce, and equity roles.

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

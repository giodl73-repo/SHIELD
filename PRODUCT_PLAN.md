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
- A deterministic CMS–USDA county rurality join covering 5,360 of 5,432
  facilities while preserving 72 unmatched rows as an explicit residual.
- A current HRSA primary-care HPSA registry census that distinguishes 79,150
  component rows, designation status, 7,682 currently designated IDs, and
  component/rural-status multiplicity.
- A separately preserved June 30, 2026 HRSA quarterly total of 9,003
  primary-care designations; different-vintage totals are not forced together.

## Next public work

The facility, rurality, and formal shortage-registry baselines are complete.
Next:

1. Build a same-vintage designation-component-geography bridge without
   converting components into whole-county or hospital shortage assignments.
2. Add an official staffed service-line capacity source only where a compatible
   geographic denominator and baseline/surge basis exist.
3. Keep county class, formal shortage designation, facility presence, staffed
   service availability, and patient-relevant travel access distinct.
4. Test a specific HLT candidate against access, quality/safety, equity,
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

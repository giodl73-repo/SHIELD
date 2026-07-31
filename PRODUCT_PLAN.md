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
- A same-vintage HRSA geography bridge separating 2,838 area designations from
  4,844 facility designations, preserving 762 multi-component and 155
  multi-county area designations, and exposing 18 invalid county-key residuals.
- A same-vintage HRSA designation-formula capacity baseline covering 2,838 area
  and 550 correctional designations, with exact FTE/shortage arithmetic and
  4,294 policy-excluded facility designations kept distinct from zero.

## Next public work

The facility, rurality, and formal shortage-registry baselines are complete.
Next:

1. Build a compatible CMS/HRSA facility-identity bridge only if explicit source
   identifiers support it; never infer identity from county co-location.
2. Add an official provider/site identity or staffed service-line source only
   where a compatible geographic denominator and baseline/surge basis exist;
   do not relabel designation-formula physician FTE as hospital staffing.
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

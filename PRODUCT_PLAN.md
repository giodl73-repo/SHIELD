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
- A CMS annual cost-report operational spine covering 5,953 valid reports and
  5,895 CCNs, including exact CCN overlap with 5,032 of 5,432 current hospitals
  and bed-day-weighted inpatient use with missing and invalid residuals.
- A Q2 2026 CMS QIES Provider of Services spine with exact CCN overlap for
  5,422 of 5,432 current hospitals, complete certified-service modes for 5,286,
  an explicit 136-hospital federal residual, and seven recorded employed-FTE
  fields that preserve zeros and conspicuous maxima.
- A CMS emergency-process spine with 4,660 exact current-hospital matches,
  six numeric process measures plus ED volume, explicit unavailable values,
  descriptive national comparisons, and a separate 41-facility Rural
  Emergency Hospital reporting surface.
- A 2024 CMS Original Medicare county emergency-demand bridge covering 3,197
  counties, 3,143 usable ED-use rates, 5,300 exactly placed current facilities,
  and 762 counties without a current hospital location, explicitly held from a
  no-access interpretation until cross-county travel is observed.
- A 2024 CMS inpatient origin-destination baseline covering 1,156,702
  hospital/beneficiary-ZIP pairs, with 5,902 exact same-year provider matches
  and 11,586,529 of 13,330,468 classified observable cases crossing a ZIP
  boundary, without treating inpatient flow as emergency access or travel time.
- A 2024 NEMSIS national EMS destination spine covering 60,298,684 activations,
  46,733,668 911 activations, and 30,123,274 destination-coded events, including
  27,863,074 routed to hospital or freestanding emergency departments, while
  preserving voluntary reporting and restricted public geography.
- A July 2026 Minnesota stroke-system access benchmark with 123 designated
  hospitals, 97% of residents within a modeled 30-minute drive, and 99% within
  60 minutes, while separating modeled statewide coverage from actual EMS
  trips, county results, national inference, need, and adequacy.

## Next work

The facility, rurality, formal shortage-registry, available-bed-use,
certified-service/workforce, and emergency-process baselines are complete.
Next:

1. Add machine-readable substate emergency access coverage or publicly usable
   patient flow linking incident origins to service locations.
2. Add total-population demand and current emergency operations before testing
   a specific access or delivery intervention.
3. Keep county class, formal shortage designation, facility presence, available
   bed use, staffed
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

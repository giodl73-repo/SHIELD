# Wave: CMS Hospital Footprint

Status: **complete**

## Goal

Execute accepted `WP-007` as SHIELD's first current public aggregate-data
feature and expose the exact held HLT handoff for Taxlane.

## Result

- Captured the CMS Hospital General Information release dated 2026-05-13.
- Reconciled 5,432 unique hospitals across hospital type, emergency-service
  flag, and 56 state/territory groups.
- Exposed `cms-access-baseline` and `cms-access-held-pack` product commands.
- Kept patient, access, capacity, quality, outcome, equity, adequacy, cost,
  savings, allocation, clinical/facility action, public-release, and rate
  claims outside the result.

## Role-review fixed point

Health-system, clinical, operations, economic/payer, equity/public-health,
citation, numeracy, and scope findings are dispositioned in
`docs/vtrace/WORK_PACKAGE_CMS_ACCESS.md`. No critical or major finding remains.

## Verification

| Check | Expected |
|---|---|
| Feature tests | four pass |
| Workspace tests | all pass |
| Formatting and clippy | clean |
| Baseline replay | deterministic JSON, denominator 5,432 |
| Held-pack replay | fourteen sections, `HLT`, admission false |

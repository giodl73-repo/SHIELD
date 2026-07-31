# WP-009: HRSA Primary-Care HPSA Registry Census

Status: **accepted**

## Objective

Produce a current, replayable census of HRSA primary-care HPSA registry grain,
status, designation type, rural-status multiplicity, and component expansion.
Compare—without merging—the current daily CSV to HRSA's June 30, 2026 quarterly
summary. This is a shortage-registry feature, not a hospital, county, patient,
workforce, or adequacy assignment.

## Source custody

| Source | Accepted custody |
|---|---|
| HRSA primary-care HPSA CSV | Created 2026-07-31; 79,150 component rows; 48,303,351 bytes; SHA-256 `8ed0007ef82194e44a1ee9086723e42f79e5effe96104cf3104648f2837d2673` |
| HRSA quarterly HPSA report | Data as of 2026-07-01 / designations as of 2026-06-30; 937,344 bytes; SHA-256 `e71178b1c88a6b00a8a4a28780ecbff1f75e39f490ea43a8e5c5c899e211f89c` |

## Grain rules

- CSV rows are designation components, not unique designations.
- `HPSA ID` is the designation identity for the current-file census.
- `Designated`, `Proposed For Withdrawal`, and `Withdrawn` remain separate.
- Geographic, population-group, and facility designations are not fungible.
- A designation may span multiple components and rural-status values.
- Designation populations may overlap and must not be summed as unique people.
- The daily CSV and quarterly report have different dates and status surfaces;
  their totals remain separate until HRSA supplies a same-vintage reconciliation.

## Product surfaces

- `data/derived/hrsa-primary-care-hpsa-census-2026-07-31.json`
- `shield hrsa-primary-care-baseline`
- `shield hrsa-primary-care-held-pack`

## Claim contract and fixed point

Allowed: source-row, unique-ID, status, designation-type, rural-status, and
component-structure counts at their declared vintages.

Held: hospital-level HPSA assignment, whole-county shortage, unique affected
population, patient access, provider supply, staffed service capacity, need met
at a facility, adequacy, candidate effect, costs, savings, allocation, or rates.

Health-system, clinical, operations, economics/payer, equity/public-health,
citation, numeracy, and scope review all require the grain and vintage rules
above. No critical or major issue remains after making them executable.
WP-009 is accepted for implementation.

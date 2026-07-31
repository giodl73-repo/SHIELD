# WP-010: HRSA Primary-Care Designation–Component–Geography Bridge

Status: **accepted**

## Objective

Convert the July 31, 2026 primary-care HPSA census into a same-vintage
designation-to-component-to-common-county-key bridge while preserving area,
facility, whole-county-component, tract, subdivision, multi-component,
multi-county, and invalid-key distinctions.

## Source and key contract

The accepted input is the WP-009 HRSA CSV: 48,303,351 bytes at SHA-256
`8ed0007ef82194e44a1ee9086723e42f79e5effe96104cf3104648f2837d2673`.
Only `Designated` rows enter this bridge. A usable common county key must be a
five-digit numeric `Common State County FIPS Code` whose first two digits equal
`Common State FIPS Code`. Invalid keys remain residuals and are not repaired
from alternative columns.

## Grain rules

- Area and facility designations remain separate.
- `SCTY`, `CT`, `CSD`, and `UNK` component classes remain separate.
- A `Single County` component does not imply that every designation has one
  component or one county.
- Tract and subdivision components retain their subcounty meaning even when a
  common county key is available.
- A facility common-county key is context, not a CMS-facility identity match.
- County-key presence does not establish whole-county shortage, patient access,
  provider capacity, or a unique affected population.

## Product surfaces

- `data/derived/hrsa-primary-care-geography-bridge-2026-07-31.json`
- `shield hrsa-geography-baseline`
- `shield hrsa-geography-held-pack`

## Claim contract and fixed point

Allowed: same-vintage counts of designations, components, component classes,
county-key multiplicity, valid common-county keys, and explicit residuals.

Held: CMS hospital or provider identity join, whole-county shortage, unique
affected population, patient access, staffed capacity, adequacy, candidate
effect, costs, savings, allocation, or rates.

Health-system, clinical, operations, economics/payer, equity/public-health,
citation, numeracy, and scope review require the rules above. No critical or
major issue remains after making the partitions and residual executable.

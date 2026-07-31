# Wave: HRSA Primary-Care HPSA Registry Census

Status: **complete**

## Goal and result

Execute accepted WP-009 by reproducing the July 31, 2026 HRSA primary-care
HPSA detail download at designation-component and unique-HPSA-ID grain.

- 79,150 component rows reconcile across registry statuses.
- 7,682 unique IDs are currently `Designated`, 1,014 are `Proposed For
  Withdrawal`, and 8,999 are `Withdrawn`.
- The nine current designation-type counts reconcile to the 7,682 designated
  IDs.
- 762 designated IDs span multiple components; the maximum is 177 component
  rows for one ID.
- 282 designated IDs span multiple rural-status values and remain explicitly
  multi-valued.
- HRSA's separate June 30 quarterly report counts 9,003 primary-care
  designations and remains a separate official vintage.

## Boundary and fixed point

CSV rows are components, not designations, hospitals, counties, or people.
Designation populations can overlap. The daily CSV and quarterly report have
different dates and status surfaces, so their 7,682 and 9,003 totals are not
reconciled. No hospital shortage, whole-county shortage, patient access,
staffed capacity, adequacy, candidate effect, cost, savings, allocation, or
rate authority is inferred.

## Verification

The feature has twelve tests and the workspace has 46. Formatting, clippy, all
workspace tests, and both CLI replays pass. The compact fourteen-section held
pack is 5,291 UTF-8 bytes without a trailing newline and has SHA-256
`1345f2c1d0aa0d1b98961cf17a07653a14e7452ecea041c19e7a047e02cd6d0a`.

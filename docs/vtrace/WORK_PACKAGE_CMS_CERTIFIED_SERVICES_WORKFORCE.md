# WP-013: CMS Certified Services and Workforce Spine

Status: **accepted**

## Objective

Establish a reproducible CMS QIES Provider of Services spine for certified
hospital service-delivery modes and provider-recorded employed FTE fields,
joined to SHIELD's current hospital footprint by exact CCN.

## Source custody

| Source | Accepted custody |
|---|---|
| CMS QIES Provider of Services Q2 2026 | Released 2026-07-16; 44,707 rows; 30,195,693 bytes; SHA-256 `bcfb9c680f02fdc05a4c82b90e434ad96bb52ea0a1f04eb424bb3dad5a9ffe3d` |
| CMS data dictionary | 6,141,361 bytes; SHA-256 `08291e4cd5d0221b1201d48be1d58a916af9ea4b608b90c4ebc5ae450ef1f4b7` |
| CMS methodology | 53,334 bytes; SHA-256 `3f9c45bc360101b8ccb0d9107a912fd0f703e9ef7cf060028b35c29fee15705b` |
| CMS Hospital General Information | May 13, 2026 footprint; SHA-256 `83c98b2e8687580e0482b13e1e9acd5813534be243e5ccd9f55556a869595d40` |

## Grain and validity rules

- Select provider-category code `01` and preserve unique six-character CCN
  facility identity.
- Join to the current CMS footprint only by exact CCN.
- Partition each service field among `not provided`, `staff`, `under
  arrangement`, `both`, and missing; do not collapse delivery modes.
- Keep the federal service-field residual explicit.
- Retain workforce fields as source-recorded employed FTE hundredths and
  preserve missing, zero, positive, negative, and maximum-value checks.
- Do not interpret workforce totals as unique people, current shifts, schedule
  coverage, vacancies, or total labor supply.

## Product surfaces

- `data/derived/cms-certified-services-workforce-2026-q2.json`
- `shield cms-certified-services-workforce-baseline`
- `shield cms-certified-services-workforce-held-pack`

## Claim contract and fixed point

Allowed: source and hospital-row identity, exact CCN overlap, certified service
delivery modes, missing federal residual, and source-recorded employed FTE
field coverage, zero counts, totals, and maxima.

Held: current schedules, operating hours, throughput, wait time, vacancies,
contract labor, unique workers, staffed service capacity, patient access, local
need, surge readiness, quality, adequacy, candidate effects, costs, savings,
allocation, rates, or public release. The work package is complete when these
boundaries and all identity, service, workforce, and join partitions are
executable.

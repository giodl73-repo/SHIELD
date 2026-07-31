# WP-012: CMS Hospital Operational-Capacity Spine

Status: **accepted**

## Objective

Establish a reproducible hospital report-period capacity and utilization spine
from CMS annual cost reports, join it to SHIELD's current hospital footprint by
exact CCN, and keep available beds distinct from staffed capacity and adequacy.

## Source custody

| Source | Accepted custody |
|---|---|
| CMS Hospital Provider Cost Report 2023 | Released 2026-01-08; 6,103 rows; 4,130,294 bytes; SHA-256 `614f3d94dfeb84092ca775f90913ab4f843233a4fa90c3df3013efeb5221a757` |
| CMS data dictionary | 377,560 bytes; SHA-256 `40342bb506d4c39ad6a3306d188f414e97e683c8c7241aa5e0975bdc59c87b81` |
| CMS methodology | 49,641 bytes; SHA-256 `e216d1f303095bdd57acec09c9f11b70c5188c8ee3460d201a5556db64d9b410` |
| CMS Hospital General Information | May 13, 2026 footprint; SHA-256 `83c98b2e8687580e0482b13e1e9acd5813534be243e5ccd9f55556a869595d40` |

## Grain and validity rules

- Preserve `rpt_rec_num` report grain and unique CCN facility identity.
- Treat repeated CCNs as separate records only after verifying their report
  periods are adjacent and non-overlapping.
- A usable record requires present adult-and-pediatric beds, available bed-days,
  and inpatient days; nonnegative beds and inpatient days; positive bed-days;
  and inpatient days no greater than available bed-days.
- Retain missing and invalid records as explicit residuals.
- Weight inpatient use by valid report-period bed-days.
- Join to the current CMS footprint only by exact CCN.
- Do not sum point-in-time bed counts across reports or call available beds
  staffed beds.

## Product surfaces

- `data/derived/cms-hospital-operational-capacity-2023.json`
- `shield cms-operational-capacity-baseline`
- `shield cms-operational-capacity-held-pack`

## Claim contract and fixed point

Allowed: report and CCN identity, required-field coverage, invalid residual,
available bed-days, inpatient days, bed-day-weighted inpatient use, reporting
period structure, and exact CCN overlap with the current footprint.

Held: staffed beds, service-line or workforce capacity, patient access, local
need, surge readiness, quality, adequacy, candidate effects, costs, savings,
allocation, rates, or public release. The work package is complete when these
boundaries and all report, validity, time, and join partitions are executable.

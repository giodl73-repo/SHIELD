# WP-015: CMS County Emergency-Demand Bridge

Status: **accepted**

## Objective

Establish a reproducible 2024 Original Medicare county emergency-use
denominator, reconcile county observations to national totals, and bridge
resident-demand counties to current hospital/service locations by exact FIPS.

## Source custody

| Source | Accepted custody |
|---|---|
| CMS Medicare Geographic Variation - National, State & County | Modified 2026-05-15; 36,994 rows; 57,865,948 bytes; SHA-256 `10c8304012da34da3ecfe4caf4548927095f693383814d0e79ce6711b6806fad` |
| CMS data dictionary | 563,924 bytes; SHA-256 `75a8d4bef07d1900a50732c78a2aec688ba3ca132dad1dc6cab1a9243d55109f` |
| CMS methodology | 196,478 bytes; SHA-256 `e7c6ca8a3cb4cd761f44ee5d5e4ee78a379479a73cd93f69fb116c860a9944ca` |

## Grain and validity rules

- Select 2024 `County` rows with age level `All`; preserve unique five-digit
  beneficiary-residence county identity.
- Preserve `*` suppression and missing values rather than filling zero.
- Reconcile valid county beneficiary and ED-visit sums to separately reported
  national totals with explicit residuals.
- Join current facility locations through exact QIES POS county FIPS.
- Keep beneficiary residence distinct from treating-facility location.
- Do not interpret utilization as unmet need or county co-location as access.

## Product surfaces

- `data/derived/cms-county-emergency-demand-2024.json`
- `shield cms-county-emergency-demand-baseline`
- `shield cms-county-emergency-demand-held-pack`

## Claim contract and fixed point

Allowed: Original Medicare beneficiary and ED-use observations at county
residence, suppression residuals, national reconciliation, rate distributions,
and current facility-location overlap by exact FIPS.

Held: total-population demand, treating hospital, patient flows, travel time,
catchments, unmet need, local adequacy, causal explanations, candidates, costs,
savings, allocation, rates, or public release. The work package is complete
when county, national, suppression, facility, and claim-boundary partitions are
executable.

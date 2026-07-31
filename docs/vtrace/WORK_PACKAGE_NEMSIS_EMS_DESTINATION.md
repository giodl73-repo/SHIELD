# WP-017: NEMSIS National EMS Destination Spine

Status: **accepted**

## Objective

Establish a reproducible national 2024 EMS activation and destination-type
baseline from the NEMSIS annual public data report while preserving voluntary
submission, restricted geography, nontransport, missing-value, and
activation-versus-patient boundaries.

## Source custody

| Source | Accepted custody |
|---|---|
| NEMSIS Data Report 2024 | Created 2025-09-24; 18 pages; 2,054,076 bytes; SHA-256 `64acb775e9b16f49427371e8b71f0dcda5dba5ec84bb92e004e1149e1f949a84` |

## Grain and validity rules

- Preserve the report's separate activation, 911, destination, transport-mode,
  and incident-urbanicity denominators.
- Reconcile each published category table independently.
- Treat activations as events, not unique patients.
- Preserve voluntary submission, the stated 99% incident-county coverage, and
  the small remote-county residual.
- Do not infer public county origin-destination flow: state/county/ZIP
  geographic identifiers are restricted on the public surface.
- Do not infer distance or time from destination type or urbanicity.

## Product surfaces

- `data/derived/nemsis-ems-destination-2024.json`
- `shield nemsis-ems-destination-baseline`
- `shield nemsis-ems-destination-held-pack`

## Claim contract and fixed point

Allowed: published national EMS activation, 911, destination-type,
transport-mode, and incident-urbanicity counts under the report's inclusion
rules.

Held: county origin-destination flow, scene-to-destination duration, road travel
time, coverage completeness, local access, unmet need, adequacy, candidates,
effects, costs, savings, allocation, rates, or public release. The work package
is complete when source, denominator, category-sum, and claim-boundary
partitions are executable.

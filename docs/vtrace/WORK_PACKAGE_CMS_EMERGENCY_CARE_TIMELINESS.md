# WP-014: CMS Emergency-Care Timeliness Spine

Status: **accepted**

## Objective

Establish a reproducible facility-level emergency-process baseline from CMS
Timely and Effective Care data, preserve unavailable values and separate Rural
Emergency Hospital reporting, and join the result to SHIELD's current hospital
and certified-service identities by exact CCN.

## Source custody

| Source | Accepted custody |
|---|---|
| CMS Timely and Effective Care - Hospital | Released 2026-05-13; 138,173 rows; 34,178,467 bytes; SHA-256 `5d39e1fd8b7b272fe83f7b53e2f69288c997dfb4d28b68dd74454e80e7d860e9` |
| CMS Timely and Effective Care - National | 45 rows; 13,805 bytes; SHA-256 `e71b0a16dc71eb9826b1d7cc4eab5c3bddbc1a285681f28e2ae33a9c2b8628e9` |
| CMS Rural Emergency Hospital provider/national | 164 / 8 rows; SHA-256 `c83bdee86d813a9a23b642cc3ed159825cef355e9f025c274f50f64cd12568e0` / `ed2328cd063920a0cd45c5ee36d13f91983657a161affaf73c0d76115203d1af` |
| CMS hospital data dictionary | 1,291,356 bytes; SHA-256 `cd5016abee26e914b273a8fea8ab698710ff60f1c53a1b66e43bbd7168f6cb81` |

## Grain and validity rules

- Preserve one facility-measure-period row and exact six-character CCN.
- Keep ED volume categorical; parse only numeric process scores.
- Preserve `Not Available` and footnote-driven residuals.
- Keep Rural Emergency Hospital measures in their separate reporting surface.
- Compare facility values to the matching CMS national value using each
  measure's stated direction; do not call the national value a target or floor.
- Do not average facility medians into a patient-weighted system result.

## Product surfaces

- `data/derived/cms-emergency-care-timeliness-2026-05.json`
- `shield cms-emergency-care-timeliness-baseline`
- `shield cms-emergency-care-timeliness-held-pack`

## Claim contract and fixed point

Allowed: exact facility identity, stated reporting periods, measure and value
availability, categorical ED volume, facility-level process score distributions,
and descriptive comparisons to matching CMS national values.

Held: live waits, current schedules or staffing, patient-weighted system waits,
travel/catchment access, local need, causal explanations, quality outcomes,
adequacy, candidates, costs, savings, allocation, rates, or public release. The
work package is complete when all identity, measure, comparison, reporting, and
residual partitions are executable.

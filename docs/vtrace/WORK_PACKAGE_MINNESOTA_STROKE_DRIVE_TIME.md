# WP-018: Minnesota Stroke-System Drive-Time Coverage

Status: **accepted**

## Objective

Establish the first current, emergency-service-specific modeled drive-time
coverage benchmark while preserving the distinction between population
coverage, actual EMS transport, substate access, need, and adequacy.

## Source custody

| Source | Accepted custody |
|---|---|
| Minnesota Stroke System landing page | Updated 2026-07-21; reports 123 designated stroke hospitals. |
| July 2026 drive-time map | 1 page; 333,247 bytes; SHA-256 `490fc7ffc2c5af244ceafb35459b9b1c59907a6b66991a972d309d9ea1c130c4`. |

## Grain and validity rules

- Preserve 97% within 30 minutes and 99% within 60 minutes as statewide
  published population shares.
- Require the 30-minute share to be no greater than the 60-minute share.
- Treat drive time as modeled population coverage, not observed ambulance or
  individual travel time.
- Do not infer county values or exact uncovered population counts from a map
  whose summary publishes neither.
- Do not generalize Minnesota stroke-system coverage to other conditions,
  states, or the nation.

## Product surfaces

- `data/derived/minnesota-stroke-drive-time-2026-07.json`
- `shield minnesota-stroke-drive-time-baseline`
- `shield minnesota-stroke-drive-time-held-pack`

## Claim contract and fixed point

Allowed: Minnesota Department of Health's published July 2026 statewide shares
within modeled 30- and 60-minute drives of 123 designated stroke hospitals.

Held: actual EMS response or transport, patient origins and destinations,
county estimates, exact uncovered counts, national access, service readiness,
unmet need, adequacy, candidates, effects, costs, savings, allocation, rates,
or public release. The work package is complete when custody, monotonic drive-
time bands, and all claim boundaries are executable.

## Role review notes

The seven-voice parliament and three-role editorial gate reached a fixed point:

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — state scale and stroke-system network scope are explicit; no facility-expansion claim. |
| Clinician / care delivery | pass — designation is not treated as staffing, readiness, safety, or outcomes. |
| Operations / capacity | pass — modeled coverage is not treated as EMS response, transport, transfer, or surge performance. |
| Health economist | pass — no intervention, value, cost, or savings claim is made. |
| Equity / access | pass — aggregate coverage is useful, while substate distribution and the uncovered population remain held. |
| Public health / prevention | pass — the condition-specific population measure is preserved without claiming outcomes or prevention effects. |
| Payer / consolidation | pass — no payment, ownership, cooperation, CON, margin, or service-survival assumption is introduced. |
| Citation auditor | pass — every published quantity resolves to the source registry and exact PDF custody. |
| Numeracy checker | pass — percent units and nested 30-/60-minute bands are distinct and monotonic. |
| Scope keeper | pass — the artifact stays within Minnesota state-scale baseline context and forbids cross-scale inference. |

No critical or major finding remains. Machine-readable substate coverage,
actual travel, demand, operations, outcomes, and candidate economics are
deferred to a later work package rather than inferred.

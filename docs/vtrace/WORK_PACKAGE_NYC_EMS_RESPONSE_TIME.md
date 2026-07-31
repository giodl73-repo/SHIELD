# WP-019: NYC EMS Response-Time Context

Status: **accepted**

## Objective

Establish a current local EMS operational timing baseline from official FDNY
dispatch aggregates while preserving the distinction between dispatch labels,
patient condition, response components, service adequacy, and outcomes.

## Source custody

| Source | Accepted custody |
|---|---|
| NYC OpenData EMS Incident Dispatch Data | Updated 2026-07-15; 29,978,154 rows at capture; incidents span 2005-01-01 through 2026-06-30. |
| FDNY data dictionary | 22,388 bytes; SHA-256 `ccf797381643e39ebcd652892c730ab180bd751bedd9519b4cd9afc0e374a9de`. |
| Five aggregate API responses | Exact query text, response byte counts, and SHA-256 values are retained in the fixture; no incident rows are stored. |

## Grain and validity rules

- Reconcile the 1,612,273 calendar-2025 incidents to 1,510,191 valid and
  102,082 invalid response-time records.
- Require borough and severity partitions to reconcile independently to all
  valid events, and severity-1 boroughs to reconcile to 27,540 events.
- Preserve response as incident creation to first unit on scene and travel as
  first-unit assignment to first unit on scene.
- Preserve seconds and arithmetic means; do not relabel them as medians,
  percentiles, live waits, targets, or scene-to-hospital transport.
- Treat severity as dispatch information, not actual patient condition.

## Product surfaces

- `data/derived/nyc-ems-response-time-2025.json`
- `shield nyc-ems-response-time-baseline`
- `shield nyc-ems-response-time-held-pack`

## Claim contract and fixed point

Allowed: official calendar-2025 aggregate counts and arithmetic mean
incident-to-scene and assignment-to-scene seconds by borough and dispatch
severity, including the 27,540 severity-1 events and their 421.713-second mean
response time.

Held: actual patient condition, specific incident locations, individual
records, scene-to-hospital time, percentiles, target compliance, population
rates, causal explanations, outcomes, need, adequacy, national inference,
candidates, effects, costs, savings, allocation, rates, or public release.
The work package is complete when source/query custody, reconciliations,
operational definitions, and every negative boundary are executable.

## Role review notes

The seven-voice parliament and three-role editorial gate reached a fixed point:

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — NYC borough context is useful, but no national or network-redesign conclusion is made. |
| Clinician / care delivery | pass — dispatch severity is not treated as actual condition, clinical readiness, safety, or outcome. |
| Operations / capacity | pass — observed response components are retained while surge, SLA, adequacy, and tail-performance claims remain held. |
| Health economist | pass — no intervention, value, cost, productivity, or savings claim is made. |
| Equity / access | pass — borough means expose distribution for follow-up without asserting cause or local inequity. |
| Public health / prevention | pass — emergency operations are observed without inferring population burden, prevention effect, or health outcome. |
| Payer / consolidation | pass — no payment, ownership, market, cooperation, or service-survival assumption is introduced. |
| Citation auditor | pass — the official dictionary and all five API responses have exact custody. |
| Numeracy checker | pass — seconds, means, validity share, partitions, and source min/max remain distinct; no median is invented. |
| Scope keeper | pass — the result stays local, aggregate, non-patient, and non-fiscal. |

No critical or major finding remains. Percentiles, targets, denominators,
outcomes, causal analysis, candidate economics, and national transfer are
deferred to later accepted work packages rather than inferred.

# WP-020: NYC EMS Response Distribution and Target Context

Status: **accepted**

## Objective

Expose severity-1 response tails from privacy-preserving aggregate counts and
preserve official NYC response targets without forcing incompatible definitions
into a false performance or adequacy finding.

## Source custody

| Source | Accepted custody |
|---|---|
| FDNY citywide response-second frequency query | 1,491 rows; 80,071 bytes; SHA-256 `deb05f69e2d5a8bd239170d277527332974226f9a27a1ec535054730822d7445`. |
| FDNY borough response-second frequency query | 4,504 rows; 340,061 bytes; SHA-256 `59d87fdcea0beeb7347b265110ef76d3ad8cf62ea5c9ae008616acc567b7be41`. |
| Preliminary Fiscal 2026 Mayor's Management Report | 12,213,302 bytes; SHA-256 `cdc002703628fbb71ae8b934c9a27654e13ace444b8300669e22792bd2b55f2f`. |

## Grain and validity rules

- Reconcile citywide and borough severity-1 frequencies to 27,540 records.
- Use nearest-rank percentiles reconstructed from exact integer-second counts.
- Require p50 no greater than p90 and p90 no greater than p95.
- Reconcile 23,922 at or below 600 seconds plus 3,618 above to the total.
- Preserve MMR actuals and targets at their published fiscal-year and indicator
  grains; do not score the calendar-year severity-1 extract against them.
- Do not relabel the 600-second share as Local Law Category 9 ALS compliance.

## Product surfaces

- `data/derived/nyc-ems-response-distribution-target-2025.json`
- `shield nyc-ems-response-distribution-target-baseline`
- `shield nyc-ems-response-distribution-target-held-pack`

## Claim contract and fixed point

Allowed: severity-1 nearest-rank response percentiles and threshold counts by
borough, plus separately reported official MMR actuals and targets.

Held: direct target comparison, pass/fail, Category 9 ALS compliance, patient
condition, population rates, outcomes, causal explanation, need, inequity,
adequacy, national inference, candidates, effects, costs, savings, allocation,
rates, or public release. The package is complete when frequency arithmetic,
percentile ordering, source custody, and incompatibility boundaries execute.

## Role review notes

The seven-voice parliament and three-role editorial gate reached a fixed point:

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — tails guide the next question but do not authorize network redesign. |
| Clinician / care delivery | pass — dispatch severity and response seconds are not treated as patient condition, quality, or outcomes. |
| Operations / capacity | pass — p50/p90/p95 expose variation; target, unit-scope, SLA, surge, and causal claims remain held. |
| Health economist | pass — no intervention, productivity, cost, value, or savings claim is made. |
| Equity / access | pass — borough tail differences are visible without asserting cause, inequity, or population exposure. |
| Public health / prevention | pass — operational tails are not converted into morbidity, mortality, or prevention effects. |
| Payer / consolidation | pass — no payment, ownership, market, cooperation, or service-survival assumption is introduced. |
| Citation auditor | pass — both API responses, the PMMR PDF, and official definitions have exact custody. |
| Numeracy checker | pass — nearest-rank method, denominators, threshold direction, and target grains are explicit. |
| Scope keeper | pass — the artifact remains local, aggregate, non-patient, non-causal, and non-fiscal. |

No critical or major finding remains. A definition-compatible Local Law 119 or
MMR extract, population/outcome evidence, operations drivers, and candidate
economics are deferred rather than inferred.

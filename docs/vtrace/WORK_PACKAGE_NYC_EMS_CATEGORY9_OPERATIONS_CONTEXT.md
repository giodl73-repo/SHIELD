# WP-023: NYC EMS Category 9 ecological operations context

Status: **accepted**

## Objective and result

Test the strongest public-data bridge between official Category 9 attainment
and observable EMS operations without inventing an incident- or patient-level
join. The official Category 9 model yields 72 calendar-2025 borough-month rows.
The FDNY dispatch extract yields 66 borough-month rows. Five named boroughs join
across all 12 months, producing 60 ecological observations.

The named-borough surfaces cover 216,463 Category 9 qualifying incidents and
1,612,266 separately defined dispatch incidents. Unspecified residuals remain
visible: 136 and 7 incidents respectively.

## Descriptive screen

Exact sufficient statistics reproduce Pearson associations between the
Category 9 share and dispatch context. The largest magnitudes are average
travel seconds (`r=-0.842`), average response seconds (`r=-0.759`), held share
(`r=-0.740`), and average dispatch seconds (`r=-0.702`). These values prioritize
questions; they do not identify a driver. The qualifying-event definitions and
incident identities differ, and borough/month confounding is uncontrolled.

## Outcome boundary

Neither surface supplies a compatible patient outcome or a shared incident key.
FDNY hospital-arrival presence is a transport-process observation, not a
clinical outcome. A definition-compatible outcome join therefore remains
blocked.

## Fixed-point review

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — signals prioritize data acquisition, not network redesign. |
| Clinician / care delivery | pass — hospital arrival and response time are not treated as patient outcomes. |
| Operations / capacity | pass — co-movement is visible while driver, staffing, fleet, posting, and capacity claims remain held. |
| Health economist | pass — no intervention, marginal effect, cost, value, or savings is inferred. |
| Equity / access | pass — borough observations are not converted into causal inequity findings. |
| Public health / prevention | pass — no morbidity, mortality, or prevention effect is claimed. |
| Payer / consolidation | pass — no payment, ownership, market, or service-survival assumption is introduced. |
| Citation auditor | pass — both exact aggregate responses and FDNY metadata have custody. |
| Numeracy checker | pass — 60 joins and both unspecified residuals reconcile; every correlation recomputes from stored moments. |
| Scope keeper | pass — ecological association, operational driver, patient outcome, causality, adequacy, and fiscal authority remain distinct. |

No critical or major finding remains.

## Product surfaces

- `data/derived/nyc-ems-category9-operations-context-2025.json`
- `shield nyc-ems-category9-operations-context-baseline`
- `shield nyc-ems-category9-operations-context-held-pack`

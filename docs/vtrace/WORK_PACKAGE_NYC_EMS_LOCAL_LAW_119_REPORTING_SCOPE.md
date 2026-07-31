# WP-022: NYC EMS Local Law 119 reporting scope correction

Status: **accepted**

## Objective and correction

Audit the enacted law rather than infer its reporting grains from the dashboard
or a code summary. Local Law 119 requires monthly and yearly reports to the
Council and Mayor, website posting, citywide results for each category, and
disaggregation by borough. It does **not** require community-district or
division reporting. WP-021's contrary deferred-boundary wording is corrected.

## Public-surface result

The official website and citywide/borough report surface are observable. The
public semantic model contains 147 distinct monthly labels from April 2014
through June 2026. Its two report pages contain 37 visuals and expose the latest
month and borough surface. The exploration does not expose a historical-month
selector, and this audit did not locate a discrete yearly report.

These observations do not prove formal submission to the Council or Mayor.
Full statutory reporting compliance therefore remains unassessed for the right
reason: unverified submission and yearly-publication custody, not nonexistent
community-district or division duties.

## Claim contract and fixed point

Allowed: the exact enacted reporting duties, official website publication,
citywide/borough surface, and monthly history present in the public model.

Held: full statutory compliance; Council/Mayor submission; discrete yearly
publication; service performance pass/fail; patient outcomes; causes; inequity;
adequacy; interventions; costs; savings; allocation; rates; or public release.

The seven-voice parliament and three-role editorial gate reached a fixed point:

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — the correction changes reporting custody, not delivery-network authority. |
| Clinician / care delivery | pass — no reporting observation is converted into patient quality or outcome. |
| Operations / capacity | pass — monthly availability is not staffing, fleet, routing, or capacity evidence. |
| Health economist | pass — no intervention, effect, cost, value, or savings follows. |
| Equity / access | pass — borough is the enacted geographic grain; no causal or inequity claim follows. |
| Public health / prevention | pass — reporting availability is not a health outcome. |
| Payer / consolidation | pass — no payment, ownership, or market inference is introduced. |
| Citation auditor | pass — enacted PDF, official page, model/exploration, and query responses have exact custody. |
| Numeracy checker | pass — 147 monthly labels reconcile inclusively from 2014-04 through 2026-06. |
| Scope keeper | pass — the prior false reporting-grain boundary is corrected without promoting full compliance. |

No critical or major finding remains after correction.

## Product surfaces

- `data/derived/nyc-ems-local-law-119-reporting-scope-2026.json`
- `shield nyc-ems-local-law-119-reporting-scope-baseline`
- `shield nyc-ems-local-law-119-reporting-scope-held-pack`

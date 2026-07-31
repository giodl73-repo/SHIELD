# WP-021: NYC EMS Local Law 119 Category 9

Status: **accepted**

## Objective and result

Replace the unmatched dispatch proxy with the strongest definition-compatible
official public result. For calendar 2025, NYC's Local Law 119 dashboard reports
the Category 9 measure at **39.26% across 216,599 qualifying incidents**. Named
borough results range from 25.65% in the Bronx to 48.29% on Staten Island.

Category 9 is the percentage of responses under 10 minutes to Advanced Life
Support medical emergencies by Advanced Life Support ambulances. This is a
threshold-attainment measure, not a stated minimum acceptable share.

## Exact custody and validation

| Surface | Accepted custody |
|---|---|
| Public Power BI conceptual schema | 6,408 bytes; SHA-256 `afae4549ef2811c72ea79f39adc12a3c4bf3b3bd22337096b7153869d2106bd7`; model `533915`. |
| Calendar-2025 citywide query | Payload 2,758 bytes / SHA-256 `e7039089906703efbdc6c09d337fb1a4ddeaf057292f4bcebbef5380b92bc2f0`; response 3,157 bytes / SHA-256 `f238bfae46fcf541002a4e76e7522c8e8cfc94d6cd3476ef1aa0166dda53e4b9`. |
| Calendar-2025 borough query | Payload 2,872 bytes / SHA-256 `bdc3cef00d7d530790fc06bc2194f0c2e6f31835b08ab15a96dff0ff511309f5`; response 4,550 bytes / SHA-256 `81f6cfb984cd4cfaa14fdf3579962d0fa8e943ce1c2cfb4c1b2c8fcf0ca9c105`. |

Borough counts reconcile exactly to 216,599. The official citywide measure is
preserved as returned by the model. An exact successful-response numerator is
not derived: multiplying the displayed measure by `NUM_CALLS` is non-integral,
and the model does not expose the measure denominator/formula.

## Claim contract

Allowed: official Category 9 definition, calendar-2025 citywide and borough
published incident counts and shares, dashboard currency, and the observation
that the public citywide/borough surface exists.

Held: full compliance with every reporting grain in NYC Administrative Code
§15-129; a legal or service performance pass/fail; an exact under-10 numerator;
comparison to the severity-1 dispatch proxy; patient outcomes; causes; inequity;
adequacy; interventions; costs; savings; allocation; rates; or public release.
Section 15-129 imposes reporting duties but does not specify a minimum Category 9
share, so no legal performance verdict is available from this rate.

## Product surfaces

- `data/derived/nyc-ems-local-law-119-category9-2025.json`
- `shield nyc-ems-local-law-119-category9-baseline`
- `shield nyc-ems-local-law-119-category9-held-pack`

## Fixed-point review

The seven-voice parliament and three-role editorial gate reached a fixed point:

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — definition-compatible performance context is available; redesign authority is not. |
| Clinician / care delivery | pass — ALS response is not converted into patient condition, care quality, or outcome. |
| Operations / capacity | pass — borough variation is visible without attributing it to staffing, fleet, routing, demand, or hospitals. |
| Health economist | pass — the observation has no intervention, marginal effect, cost, value, or savings claim. |
| Equity / access | pass — geographic differences are reported without a causal or inequity conclusion. |
| Public health / prevention | pass — response attainment is not converted into morbidity, mortality, or prevention effects. |
| Payer / consolidation | pass — no payment, ownership, market, or service-survival assumption is introduced. |
| Citation auditor | pass — official landing, definitions, current code URL, model identity, schema, payloads, and responses are named. |
| Numeracy checker | pass — borough counts reconcile; official shares are preserved; the non-integral implied numerator is not invented. |
| Scope keeper | pass — publication observation, whole-law compliance, performance standard, adequacy, and fiscal authority remain distinct. |

No critical or major finding remains. The next highest-value evidence slice is a
community-district/division completeness check and outcome/operations join,
followed by a separately bounded intervention package if evidence permits.

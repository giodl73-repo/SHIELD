# WP-024: NYC EMS Category 9 public-evidence boundary

Status: **accepted**

## Objective and decision

Inventory and test the plausible official public operations and privacy-safe
outcome sources that could advance Category 9 beyond an ecological screen.
Seven source classes were checked. The result is a stronger public source spine,
an explicit source-revision conflict, a confounding sensitivity result, and a
pre-specified acquisition design. It is **not** an intervention candidate.

The direct NYC Open Data table (`gpny-cuvw`) is the preferred replay surface:
it exposes 72 calendar-2025 Category 9 borough-month rows through a documented
API. Those rows total 216,986 incidents, 387 more than the 216,599 incidents in
the captured Power BI model. The weighted share reconstructed from rounded
Open Data rows is 39.232%, versus 39.260% in Power BI. The reason is not
established, so the sources remain separate and neither silently supersedes the
other. The Open Data field metadata also incorrectly describes monthly labels
as week-start dates; the defect is recorded rather than normalized away.

## Operations and outcome boundary

The MMR adds citywide monthly ambulance in-service hours, hospital turnaround,
life-threatening incident volume, and end-to-end response time. Its 60
calendar-2025 borough response indicator rows publish no values. The 911
end-to-end source is weekly and citywide. The incident dispatch source has
place and time but does not identify the Category 9 qualifying set. No public
staffing, unit-availability, posting, or routing table was located in the NYC
Open Data catalog.

NYSDOH SPARCS exposes privacy-safe hospital disposition information, including
de-identified inpatient patient disposition, but no EMS incident key and no
day/month identity suitable for Category 9 linkage. The quarterly ED summary
provides admission dispositions rather than patient outcomes. No NYC
cardiac-arrest outcome dataset was located in the NYC Open Data catalog.

## Confounding sensitivity

The direct Open Data rows reproduce the earlier unadjusted ecological pattern.
After unweighted two-way demeaning by borough and month, the association with
average travel time remains the largest (`r=-0.481`), while average overall
response (`r=-0.187`) and dispatch (`r=-0.091`) shrink substantially. This
identifies travel time as the highest-value **hypothesis**, not as a driver.
The populations still differ and there is no treatment, instrument, patient
outcome, or common incident identity.

The 12-month citywide screen also shows strong co-movement with end-to-end
life-threatening response time (`r=-0.909`) and a smaller positive association
with ambulance in-service hours (`r=0.676`). Those estimates are especially
vulnerable to shared clock mechanics, time trend, season, and only 12
observations. Hospital turnaround has only ten published monthly values.

## Pre-specified next evaluation

Do not select a bounded intervention from current public evidence. Reopen a
stepped or interrupted implementation design only after acquiring:

1. a stable qualifying-event and incident linkage key;
2. ALS unit-hours and availability by place and time;
3. posting or routing exposure with implementation dates;
4. hospital offload interval where relevant;
5. privacy-safe survival, clinical disposition, or neurologic outcome; and
6. a pre-period plus unaffected comparison units.

The primary acquisition target is incident-linked ALS availability/posting
exposure with privacy-safe outcomes. The estimand, exclusion rules, comparison
units, pre-period, outcome horizon, and subgroup checks must be registered
before examining an implementation effect.

## Fixed-point review

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — acquisition and evaluation design are actionable; redesign is held. |
| Clinician / care delivery | pass — response clocks and hospital dispositions are not called clinical outcomes. |
| Operations / capacity | pass — travel, availability, and offload remain prioritized hypotheses, not drivers. |
| Health economist | pass — no marginal effect, intervention cost, value, or savings enters. |
| Equity / access | pass — borough fixed effects do not establish causal inequity or distributional adequacy. |
| Public health / prevention | pass — morbidity, mortality, and prevention effects require linked outcomes. |
| Payer / consolidation | pass — no payment, ownership, or service-survival claim is introduced. |
| Citation auditor | pass — principal datasets, query sizes, hashes, vintages, and the metadata defect are explicit. |
| Numeracy checker | pass — the 387-incident difference reconciles and all stored moments recompute. |
| Scope keeper | pass — source inventory, hypothesis, driver, outcome, cause, candidate, and fiscal authority remain separate. |

No critical or major finding remains.

## Product surfaces

- `data/derived/nyc-ems-category9-public-evidence-boundary-2025.json`
- `shield nyc-ems-category9-public-evidence-boundary-baseline`
- `shield nyc-ems-category9-public-evidence-boundary-held-pack`

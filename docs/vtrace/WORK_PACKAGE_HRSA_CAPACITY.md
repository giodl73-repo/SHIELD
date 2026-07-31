# WP-011: HRSA Primary-Care Designation Capacity Baseline

Status: **accepted**

## Objective

Determine which current HRSA primary-care HPSA designations carry physician
FTE and shortage-formula values, validate the area-designation population and
ratio identities, and preserve policy-defined exclusions. This is a
designation-formula capacity baseline, not a unique workforce or hospital
staffing census.

## Source custody

| Source | Accepted custody |
|---|---|
| HRSA primary-care HPSA CSV | Created 2026-07-31; 48,303,351 bytes; SHA-256 `8ed0007ef82194e44a1ee9086723e42f79e5effe96104cf3104648f2837d2673` |
| HRSA HPSA metadata | Captured 2026-07-31; 27,297 bytes; SHA-256 `0439dbf209f0b05745e99cc0911e49262b422c564282a48b9030a6175b7ab91b` |

HRSA's June 30 quarterly report defines primary-care percent need met as
available primary-care physicians divided by physicians necessary to fall
below designation thresholds. It states that the practitioner-needed formula
does not include nurse-practitioner or physician-assistant services and excludes
specified automatic or service-based facility designations.

## Grain and formula rules

- Deduplicate repeated component rows by `HPSA ID` before evaluating fields.
- Keep area and correctional formulas separate.
- Treat missing FTE/shortage values for the other facility classes as explicit
  policy-formula exclusions, not zeros.
- Store FTE and shortage in exact ten-thousandths.
- Validate area estimated-served population against FTE × ratio goal within
  one-half person and shortage against underserved ÷ ratio goal within 0.01 FTE.
- Do not sum designation populations as unique people.
- Do not treat designation-recorded FTE as deduplicated physicians or staffed
  hospital service lines.

## Product surfaces

- `data/derived/hrsa-primary-care-designation-capacity-2026-07-31.json`
- `shield hrsa-capacity-baseline`
- `shield hrsa-capacity-held-pack`

## Claim contract and fixed point

Allowed: field coverage, designation-recorded physician FTE and shortage,
provider-ratio goals, area population formulas, derived need-met ratios, and
explicit policy exclusions at current HPSA-ID grain.

Held: unique physician, NP, PA, or patient counts; CMS facility staffing;
service-line availability; appointment access; quality; adequacy; candidate
effect; costs; savings; allocation; or rates.

Health-system, clinical, operations, economics/payer, equity/public-health,
citation, numeracy, and scope review require these rules. No critical or major
issue remains after making coverage, exclusions, formulas, and rounding
residuals executable.

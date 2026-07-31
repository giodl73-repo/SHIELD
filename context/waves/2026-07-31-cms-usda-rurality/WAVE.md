# Wave: CMS–USDA County Rurality

Status: **complete**

## Goal and result

Execute accepted WP-008 by joining SHIELD's 5,432-facility CMS denominator to
USDA ERS 2023 Rural-Urban Continuum Codes at county/county-equivalent grain.

- 5,360 facilities matched deterministically (98.67%).
- 3,456 matched facilities are in metro counties and 1,904 in nonmetro counties.
- Matched facilities occupy 967 metro and 1,486 nonmetro counties.
- 1,086 of 1,371 matched Critical Access Hospitals are nonmetro.
- 36 of 41 matched Rural Emergency Hospitals are nonmetro.
- 72 facilities remain unmatched and unallocated.

## Boundary and fixed point

RUCC classifies counties, not patients, travel, catchments, staffing, shortage,
need, service availability, quality, outcomes, or adequacy. The exact-match
residual is a result, not a defect to conceal. WP-008's full domain and
editorial review found no unresolved critical or major issue after those
boundaries became executable invariants and public wording.

## Verification

The feature has eight tests and the workspace has 42. Formatting, clippy, all
tests, and both CLI replays pass. The compact fourteen-section held pack is
4,853 UTF-8 bytes without a trailing newline and has SHA-256
`0650c290f02d3559bd5459e298dcebc244af45774ce02c57363422ffdcf387ac`.

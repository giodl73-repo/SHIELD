# Wave: HRSA Primary-Care Designation Capacity Baseline

Status: **complete**

## Goal and result

Execute accepted WP-011 by validating current HRSA primary-care physician FTE
and shortage formulas at unique-HPSA-ID grain.

- 3,388 current designations carry both FTE and shortage values: all 2,838
  area designations and 550 correctional facilities.
- Those records carry 10,635.4884 designation-recorded physician FTE and
  12,267.0916 designation-recorded shortage, yielding a derived 46.44% need-met
  ratio at this bounded current-file grain.
- The area subset records 10,327.3034 FTE and 11,498.4866 shortage (47.32%);
  correctional facilities record 308.1850 and 768.6050 (28.62%).
- All 2,838 area formulas reproduce served population within half a person and
  shortage within 0.01 FTE; one designation leaves a one-person population
  rounding residual.
- 4,294 other facility designations have no FTE or shortage formula values and
  remain policy exclusions rather than zeros.

## Boundary and fixed point

The quantities are recorded per designation and may overlap. They are not
deduplicated physicians, patients, or counties. The primary-care formula omits
nurse-practitioner and physician-assistant services and specified automatic or
service-based facility designations. It does not establish CMS hospital
identity, staffed service lines, appointment access, quality, adequacy,
candidate effects, costs, savings, allocation, or rates.

## Verification

The feature has twenty tests and the workspace has 54. Formatting, clippy, all
workspace tests, and both CLI replays pass. The compact fourteen-section held
pack is 5,284 UTF-8 bytes without a trailing newline and has SHA-256
`df2db0102e6e3e91924271e24014aa6ec8c41c8217f17682f1742630226bd18c`.

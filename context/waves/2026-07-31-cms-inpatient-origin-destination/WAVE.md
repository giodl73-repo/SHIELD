# Wave: CMS Inpatient Origin-Destination Flow

Status: **complete**

## Goal and result

Execute accepted WP-016 by turning CMS's 2024 Hospital Service Area file into
a bounded observed inpatient-flow baseline with same-year provider locations.

- 1,156,702 HSA pairs contain 146,996 numeric and 1,009,706 suppressed rows.
- Exact CCN identity joins 5,902 of 7,536 HSA providers to the Q4 2024 POS
  hospital surface.
- The exact join retains 13,330,744 observable cases; 276 cases have an invalid
  origin ZIP and remain outside the same/different-ZIP classification.
- Of 13,330,468 classified cases, 11,586,529 (86.92%) have a beneficiary
  mailing ZIP different from the hospital ZIP.

## Boundary and fixed point

This directly observes cross-ZIP inpatient use and demonstrates why facility
co-location cannot stand in for patient flow. It does not establish a county
crossing, emergency-department destination, road distance, travel time, burden,
reason for travel, unique-patient count, or access adequacy. Emergency-specific
flow or patient-relevant travel time, total-population demand, current
operations, candidates, effects, costs, savings, allocation, and rates remain
held.

## Verification

The feature crate has 39 tests and the workspace has 73. Formatting, strict
clippy, workspace tests, and both compact JSON CLI replays pass. The held pack
is 5,837 UTF-8 bytes without a trailing newline and has SHA-256
`6054914ce3a848b90c1a7c34dedbc1784994ffd7c7d1067d6c6619b50707d2d6`.

# Wave: CMS Emergency-Care Timeliness Spine

Status: **complete**

## Goal and result

Execute accepted WP-014 by turning the May 2026 CMS Timely and Effective Care
release into a bounded emergency-process baseline.

- The standard surface has seven emergency rows for each of 4,660 facilities,
  all exact current-footprint CCN matches (85.79% of 5,432).
- The 772 current IDs outside it are 635 psychiatric, 132 VA, and five
  long-term hospitals.
- OP-18a has 4,050 numeric facility values and 610 unavailable values; its
  facility median is 154 minutes versus CMS's 167-minute national value.
- OP-18d transfer time has 2,340 numeric and 2,320 unavailable values.
- All 41 Rural Emergency Hospitals use CMS's separate 164-row reporting file.

## Boundary and fixed point

These are facility process observations covering 2024 or July 2024 through
June 2025, not live waits. Facility medians are not patient-weighted system
estimates. The CMS national value is descriptive, not an access or adequacy
floor. Current operations, staffing, travel/catchments, need, causal effects,
costs, savings, allocation, and rates remain held.

## Verification

The feature crate has 32 tests and the workspace has 66. Formatting, strict
clippy, workspace tests, and both compact JSON CLI replays pass. The held pack
is 5,495 UTF-8 bytes without a trailing newline and has SHA-256
`75e690eff140e3cc50a4098f4bb624f0072e295e84ff943eccb3c56fbe3774f0`.

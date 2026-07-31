# Wave: CMS Hospital Operational-Capacity Spine

Status: **complete**

## Goal and result

Execute accepted WP-012 by turning the CMS 2023 Hospital Provider Cost Report
into a bounded available-bed-use baseline with exact current-footprint identity.

- 6,103 unique report records cover 6,040 CCNs.
- 5,953 reports covering 5,895 CCNs pass the operational-field validity rule;
  125 are missing a required field and 25 are invalid.
- Valid records contain 241,546,243 available bed-days and 151,101,088
  inpatient days, or 62.56% bed-day-weighted use.
- Exact CCN identity matches 5,144 of 5,432 current hospital IDs; 5,032 have a
  usable report, with 62.33% weighted use.
- Sixty-two repeated CCNs form 63 adjacent, non-overlapping report-period pairs.

## Boundary and fixed point

CMS reports beds available for patient use, not staffed beds. Point-in-time
bed counts are not additive across records. Utilization does not establish
service-line availability, surge readiness, access, need, quality, adequacy,
candidate effects, costs, savings, allocation, or rates. The retained missing,
invalid, current-only, and cost-report-only residuals make the baseline useful
without forcing different vintages into a false census.

## Verification

The feature crate has 24 tests and the workspace has 58. Formatting, clippy,
workspace tests, and both compact JSON CLI replays pass. The held pack is
5,593 UTF-8 bytes without a trailing newline and has SHA-256
`6d5ed0aa56834b24eb6bc92eb8bf4a1927fd34e7abe2a6724f99f1a7dce6e6e7`.

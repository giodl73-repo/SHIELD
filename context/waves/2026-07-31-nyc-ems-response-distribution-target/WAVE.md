# Wave: NYC EMS Response Distribution and Target Context

Status: **complete**

## Goal and result

Execute accepted WP-020 by exposing response tails and preserving official
target context without creating a false apples-to-oranges comparison.

- The 27,540 severity-1 records have a 366-second median, 650-second p90, and
  792-second p95 response.
- 23,922 records (86.86%) are at or below 600 seconds; 3,618 are above.
- Borough p90 response times range from 593 seconds in Brooklyn to 698 seconds
  in Queens.
- The official PMMR separately reports an FY2025 ambulance actual of 8:49 and
  FY2026 target of 6:55 for life-threatening emergencies.

## Boundary and fixed point

The extract and official benchmark share NYC geography but differ on calendar
versus fiscal year, severity 1 versus Segment 1–3, incident creation versus call
receipt, and unproved arriving-unit scope. No target miss or pass is asserted.
The extract's ten-minute share is not Local Law Category 9 ALS compliance.
Patient condition, population exposure, outcomes, causes, inequity, adequacy,
candidates, costs, savings, and national transfer remain held.

## Verification

The fixture enforces exact custody, frequency reconciliation, percentile order,
threshold arithmetic, and benchmark-incompatibility boundaries. The feature
crate has 51 tests and the workspace has 85. Formatting, strict clippy,
workspace tests, and both compact JSON CLI replays pass. The held pack is 7,684
UTF-8 bytes without a trailing newline and has SHA-256
`a4aa36e3eb761a938f0b79d62735dec6e230c6e5ccf09822c2da0c733576f270`.

The full seven-voice parliament and three-role editorial gate recorded no
unresolved critical or major finding. Their dispositions are in
`docs/vtrace/WORK_PACKAGE_NYC_EMS_RESPONSE_DISTRIBUTION_TARGET.md`.

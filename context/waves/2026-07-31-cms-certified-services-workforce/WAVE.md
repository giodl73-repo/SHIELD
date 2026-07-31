# Wave: CMS Certified Services and Workforce Spine

Status: **complete**

## Goal and result

Execute accepted WP-013 by turning the Q2 2026 CMS QIES Provider of Services
file into a bounded certified-service/workforce baseline with exact
current-footprint identity.

- 13,566 hospital rows contain 13,566 unique CCNs.
- Exact CCN identity matches 5,422 of 5,432 current hospital IDs (99.82%).
- Fourteen service modes are complete for 5,286 current hospitals (97.31%);
  the 136-row residual is 112 VA and 24 DoD hospitals.
- Seven workforce fields are present for all 5,422 matched CCNs, with recorded
  zeros and conspicuous maxima preserved.

## Boundary and fixed point

Certification records staff/arrangement delivery mode, not whether a service
is open now or sufficiently staffed. Workforce values are recorded employed
FTEs, not unique people, shift coverage, vacancies, contract labor, or total
labor supply. The result therefore does not establish access, need, quality,
adequacy, candidate effects, costs, savings, allocation, or rates.

## Verification

The feature crate has 28 tests and the workspace has 62. Formatting, clippy,
workspace tests, and both compact JSON CLI replays pass. The held pack is 5,618
UTF-8 bytes without a trailing newline and has SHA-256
`141c332f39cd4468fcc1b5279046c8ed6e564a2fa9786c95891c15527e71d2ba`.

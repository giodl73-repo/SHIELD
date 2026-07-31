# Wave: Minnesota Stroke-System Drive-Time Coverage

Status: **complete**

## Goal and result

Execute accepted WP-018 by turning Minnesota Department of Health's current
stroke-system map into a bounded condition-specific access benchmark.

- 123 designated stroke-system hospitals are reported.
- 97% of Minnesotans live within a modeled 30-minute drive.
- 99% live within a modeled 60-minute drive.

## Boundary and fixed point

The result is statewide modeled population coverage, not observed EMS trips or
patient travel. The source summary does not publish county estimates,
machine-readable geography, an exact uncovered population count, current
facility readiness, outcomes, need, or adequacy. It cannot be generalized to
other conditions, states, or the nation.

## Verification

The fixture enforces exact custody, monotonic drive-time bands, and negative
claim boundaries. The feature crate has 45 tests and the workspace has 79.
Formatting, strict clippy, workspace tests, and both compact JSON CLI replays
pass. The held pack is 5,714 UTF-8 bytes without a trailing newline and has
SHA-256 `25c75014e6fd94470a09ba21d332a47fecd45d1e16da2f2fbe2b26aea460d645`.

The full seven-voice parliament and three-role editorial gate recorded no
unresolved critical or major finding. Their fixed-point dispositions are in
`docs/vtrace/WORK_PACKAGE_MINNESOTA_STROKE_DRIVE_TIME.md`.

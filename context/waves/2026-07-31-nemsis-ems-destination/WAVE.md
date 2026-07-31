# Wave: NEMSIS National EMS Destination Spine

Status: **complete**

## Goal and result

Execute accepted WP-017 by turning the NEMSIS 2024 annual report into a bounded
national EMS destination and incident-context baseline.

- 60,298,684 activations were submitted by 14,756 agencies across 54 states
  and territories; 46,733,668 are on the report's 911 activation surface.
- The destination table contains 30,123,274 coded events after its stated
  inclusion and missing-value rules.
- Hospital emergency departments received 27,706,728 coded events and
  freestanding emergency departments 156,346; together they are 27,863,074,
  or 92.50% of destination-coded events.
- The separately grained incident-urbanicity table contains 2,652,293 rural
  and 452,100 frontier events.

## Boundary and fixed point

NEMSIS is a voluntary activation registry, not a unique-patient census.
Destination, transport-mode, and incident-urbanicity tables have different
denominators. The report states that public state/county/ZIP identifiers are
restricted and that a small number of remote counties do not contribute data.
The result therefore establishes national emergency routing context—not linked
county origins and destinations, travel duration, road travel time, local
access, unmet need, or adequacy.

## Verification

The feature crate has 42 tests and the workspace has 76. Formatting, strict
clippy, workspace tests, and both compact JSON CLI replays pass. The held pack
is 5,676 UTF-8 bytes without a trailing newline and has SHA-256
`5e4ee63c6f1651e85a24bebc150ae9d14781dc2e28f432fdfe4b34d3155c3080`.

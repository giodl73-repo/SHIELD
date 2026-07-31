# Wave: NYC EMS Response-Time Context

Status: **complete**

## Goal and result

Execute accepted WP-019 by turning official FDNY aggregate queries into a
bounded calendar-2025 local EMS operations baseline.

- 1,612,273 incidents reconcile to 1,510,191 valid response-time events and
  102,082 invalid events (93.67% valid).
- 27,540 severity-1 valid events have a 421.713-second mean response time and
  a 372.592-second mean travel time.
- Severity-1 borough mean response time ranges from 388.215 seconds in Brooklyn
  to 456.609 seconds in the Bronx.

## Boundary and fixed point

Response is incident creation to first unit on scene; travel is first-unit
assignment to first unit on scene. Dispatch severity is not actual patient
condition. These arithmetic means do not establish percentiles, target
compliance, scene-to-hospital time, outcomes, population rates, causality,
need, adequacy, candidates, costs, savings, or national performance.

## Verification

The fixture enforces exact source/query custody, validity and partition
reconciliations, operational definitions, and negative claim boundaries. The
feature crate has 48 tests and the workspace has 82. Formatting, strict clippy,
workspace tests, and both compact JSON CLI replays pass. The held pack is 6,538
UTF-8 bytes without a trailing newline and has SHA-256
`38821b16791e8dceb56fa644b442b0010634148dd7fff5fc688b1f577aaa1a55`.

The full seven-voice parliament and three-role editorial gate recorded no
unresolved critical or major finding. Their fixed-point dispositions are in
`docs/vtrace/WORK_PACKAGE_NYC_EMS_RESPONSE_TIME.md`.

# SHIELD Waves

Work is organized into small waves and pulses.

## Waves

| Wave | Status | Scope |
|---|---|---|
| `2026-06-26-vtrace-foundation` | complete | VTRACE left-side baseline, 12 stages (MISSION → WORK_PACKAGES), pulses 01–12. |
| `2026-06-26-shield-implementation` | complete | WP-001..006 product baseline; repository state reverified 2026-07-31. |
| `2026-07-31-cms-hospital-footprint` | complete | First current CMS aggregate result and held HLT handoff (WP-007). |
| `2026-07-31-cms-usda-rurality` | complete | Exact county rurality join, visible residual, and held HLT handoff (WP-008). |
| `2026-07-31-hrsa-primary-care-hpsa` | complete | Current primary-care HPSA registry census with separate quarterly benchmark and held HLT handoff (WP-009). |
| `2026-07-31-hrsa-primary-care-geography` | complete | Same-vintage area/facility component-geography bridge with explicit county-key residual (WP-010). |
| `2026-07-31-hrsa-primary-care-capacity` | complete | Same-vintage designation-formula physician FTE/shortage baseline with policy exclusions (WP-011). |
| `2026-07-31-cms-operational-capacity` | complete | CMS annual available-bed-use spine, explicit validity residuals, and exact current-footprint CCN join (WP-012). |

## Active wave

- No active wave. The next slice requires an explicit CCN-compatible staffed
  service-line, workforce, or patient-access source and a separately accepted
  work package.

## Protocol

1. Read this file.
2. Read the active wave `WAVE.md`.
3. Read the target pulse under `pulses/`.
4. Advance exactly one deliverable / work package at a time to a fixed point or to its
   work-package exit criteria.
5. Keep mission-level files free of scoring, gap, or design content, and declare scale where
   relevant.
6. Update docs and wave/pulse status.
7. Run the repo validation commands.
8. Hand off when the stage/work package is settled and validated.

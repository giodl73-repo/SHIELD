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
| `2026-07-31-cms-certified-services-workforce` | complete | CMS QIES certified-service modes and recorded employed-FTE spine with exact current-footprint CCN join (WP-013). |
| `2026-07-31-cms-emergency-care-timeliness` | complete | CMS emergency-process measures, unavailable residuals, national references, and separate Rural Emergency Hospital surface (WP-014). |
| `2026-07-31-cms-county-emergency-demand` | complete | CMS Original Medicare county emergency-use denominator and exact facility-location bridge with cross-county access held (WP-015). |
| `2026-07-31-cms-inpatient-origin-destination` | complete | CMS Medicare inpatient beneficiary-ZIP/hospital flow with same-year provider identity and emergency/travel claims held (WP-016). |
| `2026-07-31-nemsis-ems-destination` | complete | NEMSIS national 911 EMS destination and incident-urbanicity spine with restricted geography and travel time held (WP-017). |
| `2026-07-31-minnesota-stroke-drive-time` | complete | Current Minnesota designated-stroke-hospital modeled drive-time coverage, with actual EMS trips and substate inference held (WP-018). |
| `2026-07-31-nyc-ems-response-time` | complete | Calendar-2025 FDNY incident-to-scene and assignment-to-scene means, with patient condition, outcomes, adequacy, and fiscal claims held (WP-019). |
| `2026-07-31-nyc-ems-response-distribution-target` | complete | Severity-1 response tails and separately defined official MMR targets, with direct comparison, adequacy, and fiscal claims held (WP-020). |
| `2026-07-31-nyc-ems-local-law-119-category9` | complete | Official calendar-2025 Category 9 citywide and borough publication, with whole-law compliance, pass/fail, adequacy, and fiscal claims held (WP-021). |
| `2026-07-31-nyc-ems-local-law-119-reporting-scope` | complete | Enacted reporting-scope correction and public-model publication audit, with Council/Mayor submission, yearly publication, pass/fail, and fiscal claims held (WP-022). |
| `2026-07-31-nyc-ems-category9-operations-context` | complete | Sixty-row named-borough/month ecological operations bridge, with patient outcomes, drivers, causality, adequacy, and fiscal claims held (WP-023). |

## Active wave

- No active wave. The next NYC slice requires ALS-unit-level operations or
  patient outcomes with a shared qualifying-event identity; submission and
  yearly-publication custody remain a parallel reporting task.

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

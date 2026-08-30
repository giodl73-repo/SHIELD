# SHIELD Invariants

These entries summarize properties that must remain true for SHIELD source
baselines, aggregate fixtures, HLT packs, corpus entries, gap analysis, roles,
and public claims.

## SHIELD-I-01: Patient Records Never Enter Durable Evidence

**Status:** VERIFIED

**Claim:** Patient records, individual medical recommendations, clinical
determinations, licensing decisions, payer decisions, and person-level access
inference are not retained, emitted, or used as SHIELD fixture evidence.

**Why it matters:** Healthcare access analysis can become privacy-sensitive or
clinically misleading if aggregate baselines are treated as patient evidence.

**Enforcement:** README boundary language, source-custody rules,
aggregate-only public-data commands, no-authority held packs, and tests preserve
the boundary.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`context/waves/2026-07-31-cms-hospital-footprint/WAVE.md`,
`crates/shield-cms-access/src/lib.rs`, and `.roles/ROLE.md`.

## SHIELD-I-02: Source Grain Is Not Collapsed

**Status:** VERIFIED

**Claim:** CMS facility IDs, USDA county classes, HRSA component rows, HPSA IDs,
area designations, facility designations, formula-bearing designations, and
policy-excluded designations remain separate.

**Why it matters:** Collapsing source grains can convert registries and
denominators into false access, staffing, or shortage findings.

**Enforcement:** Separate CLI baselines, wave fixed points, reconciliation
tests, component/geography bridge checks, and HRSA capacity partition tests keep
source grains visible.

**Evidence:** `README.md`,
`context/waves/2026-07-31-hrsa-primary-care-hpsa/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-geography/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

## SHIELD-I-03: Residuals Are Visible, Not Repaired By Assumption

**Status:** VERIFIED

**Claim:** Unmatched CMS/USDA rows, invalid HRSA county keys, multi-component
IDs, multi-county IDs, rounding residuals, and excluded facility designations
remain explicit residuals rather than being imputed, zeroed, or allocated.

**Why it matters:** Healthcare source residuals can hide actual boundary
conditions, data limits, or non-additive populations.

**Enforcement:** Rurality, geography, and capacity wave outputs preserve
residual counts; tests assert visible residuals and policy exclusions.

**Evidence:** `README.md`,
`context/waves/2026-07-31-cms-usda-rurality/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-geography/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

## SHIELD-I-04: Capacity Formulas Are Not Workforce Or Access

**Status:** VERIFIED

**Claim:** HRSA designation-recorded physician FTE, shortage, provider-ratio,
and need-met formulas are formula baselines at HPSA-ID grain, not deduplicated
physicians, people, hospitals, appointment slots, staffed service lines, or
patient access.

**Why it matters:** Formula arithmetic can look precise enough to support
staffing or adequacy claims that the source does not authorize.

**Enforcement:** HRSA capacity baseline text, held-pack tests, policy-exclusion
partitions, and no-authority fields block workforce, access, cost, and savings
promotion.

**Evidence:** `README.md`,
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`,
`crates/shield-cms-access/src/lib.rs`, and `PRODUCT_PLAN.md`.

## SHIELD-I-05: Public HLT Packs Remain Held

**Status:** VERIFIED

**Claim:** CMS/HRSA HLT packs keep admission false, no fiscal authority, no
facility action, no clinical action, no public release, no allocation, no rates,
and no savings authority.

**Why it matters:** A compact, deterministic evidence pack can be mistaken for
admission to an external health/fiscal lane.

**Enforcement:** Held-pack schemas, fourteen-section tests, SHA-recorded wave
outputs, and no-authority assertions keep the packs non-admission artifacts.

**Evidence:** `README.md`,
`context/waves/2026-07-31-cms-hospital-footprint/WAVE.md`,
`context/waves/2026-07-31-cms-usda-rurality/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

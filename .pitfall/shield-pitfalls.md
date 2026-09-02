# SHIELD Pitfalls

These entries capture recurring healthcare-access evidence failure classes and
map them to SHIELD controls or open repo-local risks.

## SHIELD-PF-01: Facility Footprint Becomes Access Adequacy

**Status:** MITIGATED

**Pattern:** CMS facility presence, emergency-service flags, hospital type,
county rurality, or HPSA registry presence is treated as travel access,
staffed capacity, patient access, quality, adequacy, or service availability.

**Domain:** CMS hospital footprint, CMS-USDA rurality join, HRSA HPSA registry,
public summaries, adoption docs, and HLT packs.

**Detection difficulty:** Public facility and designation counts are concrete
and reproducible, while patient-relevant access needs more data, denominators,
and review.

**Structural solution:** Preserve denominator-only language, source grain,
unmatched residuals, and explicit held claims for travel time, staffing, access,
quality, adequacy, cost, savings, and facility action.

**Evidence:** `README.md`,
`context/waves/2026-07-31-cms-hospital-footprint/WAVE.md`,
`context/waves/2026-07-31-cms-usda-rurality/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

## SHIELD-PF-02: Shortage Registry Becomes People Or Hospitals

**Status:** MITIGATED

**Pattern:** HRSA component rows, HPSA IDs, designation status, area/facility
designations, or primary-care formula values are converted into deduplicated
people, clinicians, counties, CMS hospital identities, staffed service lines,
or appointment access.

**Domain:** HRSA primary-care HPSA census, geography bridge, capacity formula,
CMS/HRSA identity bridge planning, and public findings.

**Detection difficulty:** HRSA outputs are official and numerically rich, but
their row grain, statuses, components, and formula exclusions do not map
directly to care delivery.

**Structural solution:** Keep component rows, IDs, geography, formula coverage,
exclusions, and vintages separate, and reject identity joins by county
co-location.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`context/waves/2026-07-31-hrsa-primary-care-hpsa/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-geography/WAVE.md`, and
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`.

## SHIELD-PF-03: Residuals Are Filled To Make A Clean Map

**Status:** MITIGATED

**Pattern:** Unmatched county rows, placeholder keys, state-prefix mismatches,
multi-component designations, multi-county designations, formula rounding
residuals, or policy-excluded facilities are silently assigned, zeroed, or
removed.

**Domain:** CMS-USDA rurality, HRSA geography bridge, HRSA capacity formula,
corpus validation, and release summaries.

**Detection difficulty:** Clean maps and round totals are easier to explain
than visible residuals and non-additive designations.

**Structural solution:** Preserve residual counts, policy exclusions, and
non-additive source-grain notes in baselines, tests, and held packs.

**Evidence:** `context/waves/2026-07-31-cms-usda-rurality/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-geography/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

## SHIELD-PF-04: HLT Evidence Pack Becomes Fiscal Or Clinical Authority

**Status:** MITIGATED

**Pattern:** A deterministic held pack is treated as HLT admission, savings,
allocation, facility action, clinical action, public release, rate change,
payer decision, or healthcare policy instruction.

**Domain:** CMS/HRSA held packs, Taxlane handoff, public README claims,
adoption docs, and downstream portfolio reuse.

**Detection difficulty:** The held packs are compact, deterministic, and
schema-shaped, so they can look like accepted external evidence instead of
non-authoritative review inputs.

**Structural solution:** Preserve fourteen-section held contracts, admission
false, fiscally empty/no-authority fields, and role-review boundaries in every
pack and summary.

**Evidence:** `README.md`,
`context/waves/2026-07-31-cms-hospital-footprint/WAVE.md`,
`context/waves/2026-07-31-cms-usda-rurality/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

## SHIELD-PF-05: Aggregate Baselines Become Public Access Finding

**Status:** MITIGATED

**Pattern:** The complete CMS/USDA/HRSA public-source baselines are presented
as a healthcare-access model, shortage map, hospital-staffing finding,
adequacy finding, intervention target, savings claim, HLT admission, or
release-ready public product.

**Domain:** README, SHOWCASE/public summaries, adoption path, aggregate
baselines, held packs, customer distribution, and downstream infrastructure
portfolio scoring.

**Detection difficulty:** The baselines are reproducible, validated, and useful,
and the repo now has a complete fixture-backed implementation surface; readers
may miss that access, staffing, need, adequacy, travel time, service-line,
quality, cost, and release review remain future or held.

**Structural solution:** Keep aggregate-only and no-authority language visible,
require the next public-access claim to cite source grain, scale, demand basis,
allowed/blocked claims, held interpretations, downstream owner acceptance, and
full parliament/editorial review, and block facility/clinical/fiscal
interpretation until explicit source and role evidence exists. SHIELD now uses
the public access claim boundary and `tests/check-public-access-claim-boundary.ps1`
to keep that gate visible across adoption, VTRACE, roles, invariants, and public
README surfaces.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/adoption/README.md`,
`docs/adoption/public-access-claim-boundary.md`,
`docs/vtrace/VERIFICATION.md`, `docs/vtrace/REVIEW.md`, `.roles/ROLE.md`, and
`tests/check-public-access-claim-boundary.ps1`.

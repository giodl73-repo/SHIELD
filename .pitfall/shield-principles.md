# SHIELD Principles

These entries summarize durable SHIELD decision rules for healthcare access,
aggregate public data, scale, demand basis, evidence labels, and no-authority
public outputs.

## SHIELD-P-01: Facility Presence Is Not Care Access

**Status:** ACTIVE

**Statement:** A hospital, emergency-service flag, clinic, provider count, HPSA
designation, or county class is not patient access unless reachability,
staffing, service line, coverage, affordability, continuity, quality, and demand
basis are also supported.

**Rationale:** Healthcare capacity is not fungible; the wrong bed, clinician,
specialty, payer pathway, referral route, or travel time cannot substitute for
the needed care.

**Decision rule:** Public baselines may establish denominators and source
spines, but access, adequacy, staffing, shortage, cost, savings, and action
claims remain held unless the specific evidence path supports them.

**Evidence:** `README.md`, `CLAUDE.md`, `PRODUCT_PLAN.md`,
`docs/vtrace/REQUIREMENTS.md`, and `.roles/ROLE.md`.

## SHIELD-P-02: Aggregate And Public Sources Only

**Status:** ACTIVE

**Statement:** SHIELD uses aggregate public-source baselines and synthetic
fixtures; it does not ingest patient records, emit individual medical advice,
or infer individual access from aggregate tables.

**Rationale:** Healthcare access evidence can become sensitive or misleading
when person-level records, small cells, clinical decisions, or identity joins
enter a research tooling repo.

**Decision rule:** Any artifact that would require patient records, clinical
recommendations, licensing, payer action, or individual inference remains out of
scope or held.

**Evidence:** `README.md`, `CLAUDE.md`, `data/sources.md`,
`docs/vtrace/CODE_RIGOR.md`, and `crates/shield-cms-access/src/lib.rs`.

## SHIELD-P-03: Grain And Vintage Stay Visible

**Status:** ACTIVE

**Statement:** CMS facilities, USDA county classes, HRSA component rows,
HPSA IDs, area/facility designations, formula-bearing records, and official
quarterly totals keep their source grain and date/vintage.

**Rationale:** Different public healthcare sources answer different questions
and can share labels without sharing denominators or dates.

**Decision rule:** SHIELD must not force same-looking counts to reconcile across
vintages, collapse component rows into people or counties, or infer facility
identity from geography.

**Evidence:** `README.md`, `data/sources.md`,
`context/waves/2026-07-31-hrsa-primary-care-hpsa/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-geography/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

## SHIELD-P-04: Baseline And Surge Demand Are Different

**Status:** ACTIVE

**Statement:** Capacity and adequacy claims require an explicit demand basis,
and baseline service posture cannot substitute for surge, diversion, transfer,
or continuity resilience.

**Rationale:** Average capacity can look adequate while peak demand, transfer
strain, staffing, referral discontinuity, or service-line mismatch breaks care
delivery.

**Decision rule:** Demand basis, tier/SLA posture, transfer-strain status, and
null/transfer findings remain labelled through corpus, score, tier, gap, and
review output.

**Evidence:** `CLAUDE.md`, `docs/vtrace/CODE_RIGOR.md`,
`crates/shield-corpus/src/lib.rs`, `crates/shield-network/src/lib.rs`, and
`crates/shield-gap/src/lib.rs`.

## SHIELD-P-05: HLT Handoff Is Held Evidence, Not Authority

**Status:** ACTIVE

**Statement:** SHIELD can prepare health-access evidence packs for external
review, but it cannot authorize HLT admission, savings, facility action,
clinical action, public release, allocation, rates, or payer decisions.

**Rationale:** Evidence packaging and healthcare policy, payment, clinical, and
operational decisions have different authorities and review obligations.

**Decision rule:** Held packs must preserve admission false/no-authority fields
and keep fiscal, clinical, facility, payer, allocation, and release claims empty
unless an external accepted process owns them.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`context/waves/2026-07-31-cms-hospital-footprint/WAVE.md`,
`context/waves/2026-07-31-hrsa-primary-care-capacity/WAVE.md`, and
`crates/shield-cms-access/src/lib.rs`.

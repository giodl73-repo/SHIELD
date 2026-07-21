# Mission

## Scope

Repo: SHIELD

VTRACE adoption scope: establish the mission baseline for SHIELD before creating requirements, specification baselines, trace rows, or work packages. This file is the leftmost VTRACE artifact for the repo and anchors later `REQ-*`, `SPEC-*`, `WP-*`, verification, and validation records. SHIELD is greenfield and code-free: this mission defines intent ahead of any implementation, and any implementation must be built by implementation automation from accepted work packages and trace back to the needs and constraints below.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | SHIELD shall turn public healthcare delivery data (e.g. HRSA HPSA/MUA shortage areas, CMS Provider of Services and Care Compare/Hospital Compare, AHA Annual Survey where public, Census/ACS demographics and insurance coverage, CDC PLACES/population-health data, County Health Rankings, Dartmouth Atlas HRR/HSA) into a reproducible scored corpus of existing healthcare-network elements. | A maintainer can regenerate the active corpus, score, and gap artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | SHIELD shall identify and explain healthcare delivery gaps — access/travel-time and appointment availability, bed/provider/slot capacity, quality/outcomes, referral/transfer continuity, surge resilience, workforce shortage, affordability/coverage barriers, service breadth, facility condition, public-health impact, equity/disparities, benefit-cost, and tier-SLA shortfalls — without overstating the evidence or hiding the demand basis. | Every material claim is tied to a data artifact, command, source label, confidence label, review record, scale, and declared demand basis (`Surge` vs `Baseline`) where capacity or adequacy is asserted. | accepted |
| NEED-003 | SHIELD shall convert analysis into defensible conceptual Healthcare 2.0 upgrade options, not clinical studies, facility-licensing or accreditation determinations, payer/coverage or CON determinations, medical advice, or advocacy briefs. | Proposed projects and feature packages are labelled implemented, heuristic, simulated, planned, held, or deprecated, with the demand basis (`Surge` vs `Baseline`), payer/market assumptions, and economic basis labelled before publication. | accepted |
| NEED-004 | SHIELD shall keep network identity stable as analysis moves from raw facilities/providers/service lines/referral paths/catchments to scored networks, gap regions, and design proposals. | Element-bearing artifacts join through a stable facility/provider/service-line/pathway/catchment/network identifier rather than a transient label, operator, payer contract, or map id. | accepted |
| NEED-005 | SHIELD shall expose healthcare delivery tradeoffs through adversarial review roles instead of hiding them behind a single score. | Parliament and editorial reviews can change claims, labels, next evidence steps, or promotion status. | accepted |
| NEED-006 | SHIELD shall report a rigorous null result, or a non-transferring dimension from physical-lifeline methodology, as a valid finding. | When the scored corpus shows a healthcare network is already accessible, staffed, continuous, affordable, equitable, and resilient — or that a dimension does not transfer cleanly — the artifacts say so rather than manufacturing a gap. | accepted |
| NEED-007 | SHIELD shall classify each element into a four-tier hierarchy (T1 Quaternary / Academic Medical Center, T2 Tertiary / Regional Hospital, T3 Community Hospital / Secondary Care, T4 Primary Care / Clinic) and define access time, capacity, service breadth, and outcomes SLAs per tier, so that "is healthcare service adequate here?" is answered against an explicit tier promise. | Every analyzed element carries a tier and a declared SLA, and adequacy claims are made against the tier SLA rather than an unstated baseline. | accepted |
| NEED-008 | SHIELD shall apply the same methodology at multiple scales — international (cross-border/global health-system benchmarking), national (national health system/program), regional (a hospital referral region, catchment, or state), and local (a single facility, clinic, or service line) — with every element tagged by scale and market/jurisdiction, and analysis runnable at a chosen scale. | Every corpus element declares a scale; scores, tiers, and gaps are interpreted within scale; a gap run can target a single scale without cross-scale leakage. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| SHIELD maintainer | Know which commands, artifacts, and review gates define the current truthful repo state at a given scale. | A clean validation bundle runs and the resulting artifacts match the documented claims and declared scale. |
| Healthcare access / services analyst | Inspect scored networks, gaps, and evidence labels without reverse-engineering the implementation. | Scores, gap maps, and reports cite their source surfaces, confidence posture, demand basis, and scale. |
| Health-system / referral-region planner | Understand why a network, tier, or project is supported, held, or downgraded. | Each claim names the data, scenario, role review, scale, and next evidence step that governs it. |
| Operations / utilization reviewer | See how SHIELD handles staffed capacity, ED boarding, diversion, transfers, clinic backlogs, and surge conceptually. | Capacity and adequacy claims expose their demand basis (`Surge` vs `Baseline`) and evidence level, not just an aggregate score. |
| Payer / consolidation stakeholder | See whether payer mix, reimbursement, consolidation, CON law, and margin pressure are represented honestly. | Payer, market-power, ownership, CON, and financial-sustainability assumptions are explicit and priced, not assumed free. |
| Patient / public-health / underserved reviewer | See access, affordability, disparities, prevention, and population outcomes before a project is promoted. | Travel-time, wait-time, affordability, shortage-area, outcome, and prevention claims point to data or held evidence, not narrative alone. |
| Coding agent | Make scoped changes without drifting claims, artifacts, scale, demand basis, stress-test honesty, or review obligations. | Work packages name parent IDs, affected modules/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

SHIELD will be a data corpus, review system, and research/design process for Healthcare Access 2.0, with any implementation built later by implementation automation from accepted VTRACE work packages. It is **multi-scale by design**: the same corpus, dimension pool, and tier model apply to a clinic, emergency department, service line, hospital, referral/catchment region, national health program, or international health-system benchmark, and a run targets a stated scale. Work happens inside a dirty portfolio checkout, so repo-local changes must stay scoped and must not depend on TRACKER-relative paths for build correctness. SHIELD is not yet a TRACKER submodule until intake completes.

This mission file does not yet assert any scored result. It creates the VTRACE anchor that later requirements, specifications, and work packages trace back to.

The tiering frame (NEED-007) and the scale frame (NEED-008) extend the portfolio pattern shared with ROUTE, PYLON, GAUGE, BASIN, PACKET, TARMAC, HARBOR, and DRAIN. SHIELD is the deliberate generalization stress-test beyond physical lifelines: prior siblings score physical networks whose edges carry measurable physical flows, while SHIELD tests whether scored-corpus → gap methodology remains useful when nodes are care facilities/providers, edges are referral/transfer/catchment pathways, and capacity is beds/providers/slots constrained by workforce, payment, trust, and market structure.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | SHIELD public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Element-bearing artifacts must preserve stable facility/provider/service-line/pathway/catchment/network identity; operators, payer contracts, and map ids are not primary keys. | Keeps scores, gaps, and proposals tied to stable service and jurisdictional identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and human/owner review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | SHIELD implementation changes belong in this repo; TRACKER receives only intentional submodule pointer updates after intake. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | SHIELD must not claim clinical-study findings, medical advice, construction readiness, facility-licensing/accreditation validity, payer/coverage determination, CON determination, or official agency/health-system/payer/provider endorsement. | Keeps the project framed as research, tooling, review, and conceptual design. | accepted |
| CON-007 | Every claim must declare its scale, and scores/tiers/gaps must not be compared or aggregated across scales without an explicit, labelled cross-scale note. | Prevents misleading mixing of local, regional, national, and international evidence (NEED-008). | accepted |

## Non-Goals

- SHIELD is not a clinical study, facility-licensing review, accreditation determination, or medical advice.
- SHIELD is not a payer/coverage, reimbursement-rate, Certificate-of-Need, or market-conduct determination.
- SHIELD is not an advocacy brief for a specific hospital, clinic, health system, payer, technology, or policy.
- SHIELD does not predict what CMS, states, payers, providers, or health systems will build or call.
- SHIELD does not treat illustrative maps or heuristic forecasts as proof-grade evidence unless their evidence level says so.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover corpus reproducibility, evidence posture, design boundaries, identity, review roles, null-result discipline, non-transfer findings, tiered SLAs, multi-scale applicability, and named demand basis. | Cross-check against `README.md`, `PRODUCT_PLAN.md`, and `CLAUDE.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |
| The physical-lifeline stress-test remains honest. | Requirements and review records must allow a null result or a non-transferring dimension to close as a finding instead of forcing a positive gap. | future `REVIEW.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent; it asserts no scores, gap findings, or design proposals, and names the multi-scale rule. | pass |
| Citation Auditor | Mission makes no quantitative claims beyond ID/tier labels and the ≤20 hypothesis; public source families are named as future corpus inputs. | pass |
| Numeracy Checker | Mission contains no arithmetic, capacity-rate, wait-time, outcome-rate, or cost claims. | pass |
| Health-System Planner | Mission names access, referral continuity, tiering, multi-scale, resilience, and public-interest intent. | pass |
| Operations & Capacity Officer | Mission requires demand-basis framing for capacity/adequacy (`Surge` vs `Baseline`) in NEED-002/003 and the operations user lens. | pass |
| Payer & Consolidation Realist | Initial draft underplayed payer mix, consolidation, CON law, and margin pressure; resolved by adding the payer/consolidation user lens, NEED-003 payer/market assumptions, and CON-006 determination boundary. | resolved |
| Equity, Public-Health & Clinician advocates | Mission names shortage areas, affordability, disparities, workforce, quality/outcomes, prevention, and staffed care as first-class via users and NEED-002. | pass |

Fixed-point note: one actionable finding (payer-mix/consolidation/CON/margin constraints under-represented) was raised and applied. No unresolved critical or major finding remains. Deferred: dimension pool, scoring rubric, tier SLA thresholds, demand methodology (`Surge` / `Baseline`), payer/market constraint schema, stress-test null-result handling, and the scale-tagging schema to REQUIREMENTS and SPECIFICATION_BASELINE.

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `CLAUDE.md`
- `.roles/ROLE.md`

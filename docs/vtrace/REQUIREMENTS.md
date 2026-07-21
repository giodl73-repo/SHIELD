# Requirements

## Scope

Repo: SHIELD

VTRACE adoption scope: derive initial repo-level requirements from `docs/vtrace/MISSION.md` and `docs/vtrace/CONOPS.md`. These requirements describe what SHIELD must satisfy as analysis and implementation proceed; they do not by themselves authorize implementation work — that comes from accepted work packages. Requirements stay at contract level and assert no scores or designs.

## Requirement Table

| ID | Requirement | Parent Need / Constraint / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | SHIELD shall maintain a documented regeneration path for the active corpus, score, tier-SLA, and gap artifacts from public source data. | NEED-001 / CON-003 / OPS-001 | Reproducibility is the minimum condition for trusting generated claims. | must | SHIELD maintainer | inspection / command review | accepted |
| REQ-002 | SHIELD shall label every material quantity with an evidence posture (implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited). | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | Evidence labels prevent proxy or planned work from reading as proof. | must | SHIELD maintainer | artifact inspection / review | accepted |
| REQ-003 | SHIELD shall cite a declared source in `data/sources.md` for every quantity in a corpus entry, or mark it as a labelled proxy/heuristic. | NEED-001 / CON-003 / CON-004 / OPS-001 | Uncited numbers cannot be audited or regenerated; surge and baseline bases differ. | must | data steward | citation audit / inspection | accepted |
| REQ-004 | SHIELD shall identify each element by a stable facility/provider/service-line/pathway/catchment/network identifier, not by a transient label, operator, payer contract, or map id. | NEED-004 / CON-002 / OPS-001 | Stable service and jurisdictional identity is required before scores, gaps, and proposals can be compared. | must | SHIELD maintainer | schema check / inspection | accepted |
| REQ-005 | SHIELD shall hold or reject any corpus or gap artifact that lacks a stable element identifier, a declared source label, or a declared scale. | NEED-004 / NEED-008 / CON-002 / CON-004 / CON-007 / OPS-001 | Mutable labels, uncited rows, and untagged scale cannot safely join across analysis stages. | must | SHIELD maintainer | gate / data inspection | accepted |
| REQ-006 | SHIELD shall calibrate its scoring rubric from observed corpus variance, correlation, and transfer-suitability review, and record the rubric version and rationale for each change. | NEED-002 / NEED-005 / OPS-002 | Calibration must be evidence-driven and auditable, not asserted; non-transferring dimensions must be visible. | must | SHIELD maintainer | calibration record / version diff | accepted |
| REQ-007 | SHIELD shall ground every capacity or adequacy claim in an explicit demand basis — peak **Surge** demand vs average **Baseline** steady-state — and name the basis on the claim. | NEED-002 / CON-001 / OPS-003 / OPS-006 | A bed, provider, slot, referral, or adequacy claim is meaningless without stating whether it is surge/peak or baseline/average. | must | operations reviewer | inspection / review | accepted |
| REQ-008 | SHIELD shall record a healthcare delivery network that is already accessible, staffed, continuous, affordable, equitable, resilient, and tier-conformant — or a dimension that does not transfer cleanly — as a valid null/transfer finding rather than manufacturing a gap. | NEED-006 / CON-001 / OPS-003 | Silent scope expansion to rescue a hypothesis is forbidden; stress-test failure is evidence. | must | SHIELD maintainer | gap-artifact inspection / review | accepted |
| REQ-009 | SHIELD shall route every promotable network or project claim through the 7-voice parliament and the 3-role editorial gate before downstream use. | NEED-005 / CON-001 / OPS-004 | SHIELD's review system is part of the evidence model, not decoration. | must | review steward | review inspection | accepted |
| REQ-010 | SHIELD shall represent access, capacity, quality/outcomes, referral continuity, resilience, workforce, affordability/coverage, service breadth, facility condition, public-health impact, equity/disparities, benefit-cost, tier-SLA conformance, payer mix, consolidation, Certificate-of-Need, market power, and margin constraints in reviews or claim labels before a design option is promoted. | NEED-003 / NEED-005 / OPS-004 | These stakeholder lenses must remain first-class, per the mission users and payer/consolidation role. | should | review steward | role review / inspection | accepted |
| REQ-011 | SHIELD shall keep its outputs framed as research, tooling, review, and conceptual design — not clinical-study findings, medical advice, facility-licensing/accreditation validity, payer/coverage determination, CON determination, or agency/health-system/payer/provider endorsement. | NEED-003 / CON-006 / OPS-004 | Scope control protects SHIELD from overclaiming public authority. | must | SHIELD maintainer | editorial review | accepted |
| REQ-012 | SHIELD shall keep implementation and VTRACE changes scoped to the SHIELD child repo until an intentional TRACKER submodule pointer update after intake. | CON-005 / OPS-005 | TRACKER is the snapshot repo; SHIELD owns implementation history. | must | SHIELD / portfolio maintainer | status / submodule review | accepted |
| REQ-013 | SHIELD shall advance VTRACE deliverables one at a time to a `.roles` review fixed point, recording dispositions and deferrals. | NEED-005 / OPS-005 | The one-at-a-time discipline keeps each artifact reviewable and traceable. | must | SHIELD maintainer | wave ledger / review notes | accepted |
| REQ-014 | SHIELD shall classify every analyzed element into exactly one tier (T1 Quaternary/Academic Medical Center, T2 Tertiary/Regional Hospital, T3 Community Hospital/Secondary, T4 Primary Care/Clinic) and attach the tier's declared SLA (access time, capacity, service breadth, outcomes). | NEED-007 / CON-002 / OPS-006 | A tiered SLA system requires every element to carry a tier and a promise it is judged against. | must | SHIELD maintainer | schema check / inspection | accepted |
| REQ-015 | SHIELD shall assess each element against its tier SLA and report any tier-SLA shortfall as a gap before a market is described as adequate. | NEED-007 / NEED-002 / NEED-006 / OPS-003 / OPS-006 | Adequacy must be measured against an explicit tier promise; SLA gaps are first-class findings. | must | SHIELD maintainer | gate / gap-artifact inspection | accepted |
| REQ-016 | SHIELD shall tag every element with a scale (international/national/regional/local) and market/jurisdiction, interpret scores/tiers/gaps within scale, and require an explicit labelled note for any cross-scale comparison or aggregation. | NEED-008 / CON-007 / OPS-007 | The multi-scale methodology is only sound if scale is explicit and not silently mixed. | must | SHIELD maintainer | schema check / gate / review | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need, constraint, or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Role Review Notes

| Role Lens | Requirement Impact | Disposition |
|---|---|---|
| Scope Keeper | Requirements stay at contract level; REQ-016 makes scale a hard requirement; REQ-008 makes transfer findings valid. | pass |
| Citation Auditor | Requirements introduce no new numeric claims; REQ-003 hardens citation discipline. | pass |
| Numeracy Checker | No calculations, units, scores, bed/provider rates, wait times, outcome rates, or cost claims. | pass |
| Health-System Planner | Connectivity, tiering, and multi-scale intent preserved via REQ-014/016/010. | pass |
| Operations Officer | Initial draft left the demand basis implicit; resolved by adding REQ-007 (Surge vs Baseline named on the claim). | resolved |
| Payer & Consolidation Realist | Payer mix, consolidation, CON law, market power, and margin constraints represented before promotion (REQ-010/011). | pass |
| Equity, Public-Health & Clinician advocates | Access, affordability, outcomes, workforce, service breadth, and disparities required before promotion (REQ-010). | pass |

Fixed-point note: one actionable finding (demand basis implicit) was raised and applied as REQ-007. No unresolved critical or major finding remains.

## CONOPS Trace Review

| Scenario ID | Requirements Derived |
|---|---|
| OPS-001 | REQ-001, REQ-002, REQ-003, REQ-004, REQ-005 |
| OPS-002 | REQ-006 |
| OPS-003 | REQ-007, REQ-008 |
| OPS-004 | REQ-002, REQ-009, REQ-010, REQ-011 |
| OPS-005 | REQ-012, REQ-013 |
| OPS-006 | REQ-014, REQ-015 |
| OPS-007 | REQ-005, REQ-016 |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| DEF-001 | Exact per-dimension anchors and weights. | `SPECIFICATION_BASELINE.md` and first corpus-calibration wave. |
| DEF-002 | Whether capacity scoring can combine staffed beds, providers, appointment slots, service-line availability, surge beds, and transfer capacity without hiding non-fungibility. | `SPECIFICATION_BASELINE.md` and first corpus wave. |
| DEF-003 | Specific data-source acquisition commands and refresh cadence. | `data/sources.md` and `VERIFICATION.md`. |
| DEF-004 | Implementation interfaces (CLI, schemas, crates). | `ARCHITECTURE.md` / `INTERFACES.md` after the minimum slice. |
| DEF-005 | Whether scale is a flat tag or a nested hierarchy (a local clinic within a regional referral region within a national/international benchmark). | `SPECIFICATION_BASELINE.md` / `INTERFACES.md`. |

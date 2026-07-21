# Concept of Operations

## Scope

Repo: SHIELD

VTRACE adoption scope: describe the operating scenarios that SHIELD's requirements and specification baseline must preserve. CONOPS is the bridge from mission needs (`MISSION.md`) to observable repo workflows. It asserts no scores, gaps, projects, clinical findings, facility determinations, payer determinations, or endorsed upgrades. SHIELD is greenfield and code-free, so these scenarios describe intended operation, and each names the mission need it serves. Every scenario is **scale-aware**.

## Actors

| Actor | Responsibility | Needs |
|---|---|---|
| SHIELD maintainer | Own repo truth, active goal, generated artifacts, and promotion posture at a stated scale. | Clear commands, evidence labels, review gates, scoped child-repo changes. |
| Coding agent | Make bounded changes to corpus, data, docs, and later code from accepted work packages. | Parent `NEED-*`/`REQ-*`/`WP-*` IDs, affected surfaces, validation commands, stop conditions. |
| Review steward | Run `.roles` review lanes; record changes to claims, holds, labels, or next steps. | Mission/requirement IDs, review scope, scale, artifacts to inspect. |
| Healthcare delivery analyst | Inspect corpus scores, gap maps, tier-SLA gaps, and design options at a chosen scale. | Reproducible artifacts with source, confidence, evidence posture, demand basis, payer/market constraints, and scale. |
| Data steward | Maintain `data/sources.md` and source/proxy/heuristic label discipline. | Stable source identifiers, update cadence, citation rules. |
| Stakeholder reviewer | Apply health-system, clinician, operations, health-economics, equity, public-health, and payer/consolidation lenses. | Claims that name access, capacity, quality/outcomes, referral continuity, resilience, workforce, affordability/coverage, service breadth, tier SLA, demand basis, and market constraints. |

## Scenarios

### OPS-001: Add And Score An Existing Network (serves NEED-001, NEED-002, NEED-004, NEED-008)

Trigger: a maintainer or agent adds an existing healthcare delivery element to the corpus.

Inputs: public source data (HRSA HPSA/MUA shortage areas, CMS Provider of Services and Care Compare, AHA Annual Survey where public, Census/ACS demographics and insurance coverage, CDC PLACES, County Health Rankings, Dartmouth Atlas HRR/HSA), the dimension pool, the corpus schema, `data/sources.md`, and the element's **scale** and jurisdiction.

Normal path:

1. Create one corpus file with a stable facility/provider/service-line/pathway/catchment/network identifier and a declared `scale` (international/national/regional/local).
2. Populate dimension values, each citing a source or labelled proxy/heuristic.
3. Score the element against the calibrated rubric, interpreted within its scale.
4. Record source, confidence, scale, evidence label, unit, tier, payer/market constraint, and demand basis where capacity or adequacy is asserted.

Failure or degraded path: if a source, stable id, payer/market constraint, or scale is missing, the row remains held with a source-needed label and a next evidence step; it is not filled with uncited prose or an assumed scale.

Outputs: one scored corpus entry with preserved labels and declared scale.

Handoffs: maintainer to review steward when evidence posture changes.

Validation evidence: PROOF/doc checks, source-label inspection, future `EVID-*`.

### OPS-002: Calibrate The Rubric (serves NEED-002, NEED-005)

Trigger: enough healthcare delivery networks are scored to test which dimensions differentiate.

Inputs: scored corpus, dimension pool, variance/correlation review, and transfer-suitability review within scale.

Normal path:

1. Inspect per-dimension variance and cross-dimension correlation.
2. Retire low-variance, redundant, or non-transferring axes; promote informative ones.
3. Bump the rubric version and record the rationale.

Failure or degraded path: if the corpus is too small, unbalanced, proxy-heavy, or a dimension does not transfer cleanly from physical-lifeline methods, the pass is deferred or recorded as a finding rather than forcing a score.

Outputs: a versioned rubric and a calibration record.

Handoffs: maintainer to analyst and review steward.

Validation evidence: calibration record, rubric version diff, future `EVID-*`.

### OPS-003: Build The Gap Map And Surface A Candidate (serves NEED-002, NEED-006)

Trigger: the calibrated corpus is plotted to find under-served regions.

Inputs: scored corpus, calibrated rubric, gap-analysis method, chosen scale, and declared demand basis for capacity/adequacy claims.

Normal path:

1. Plot scored networks in the dimension space at the chosen scale.
2. Identify under-served regions (e.g. long travel or appointment wait + thin provider supply + weak referral continuity + fragile surge capacity).
3. Record candidate gaps with the dimensions, corpus comparison, scale, units, tier-SLA shortfall, payer/market constraint, and demand basis that define them.
4. Record any dimension-transfer finding where referral/catchment edges, human trust, or non-fungible capacity prevents a physical-network metric from carrying over cleanly.

Failure or degraded path: if a healthcare delivery network is already accessible, staffed, continuous, affordable, equitable, and resilient at its declared tier and scale — or if a dimension does not transfer cleanly — that null or transfer result is recorded as a finding (NEED-006), not manufactured into a gap.

Outputs: gap-map artifact and candidate-gap records, or a recorded null/transfer finding.

Handoffs: maintainer to design author.

Validation evidence: gap artifact, reproduction command, future `TRACE.md`.

### OPS-004: Review And Promote Or Hold A Design Claim (serves NEED-003, NEED-005)

Trigger: a conceptual Healthcare 2.0 project/feature package is proposed for downstream use.

Inputs: the proposal, evidence labels, scale, demand basis, payer/market constraints, parliament/editorial lenses, and non-goal constraints from `MISSION.md`.

Normal path:

1. Confirm the claim carries an evidence label, declared scale, tier/SLA context, payer/market assumptions, and relevant demand basis.
2. Run the 7-voice parliament; require each voice to challenge or accept â€” including the Payer & Consolidation Realist on payer mix, Medicare/Medicaid/commercial/uninsured coverage, hospital/practice consolidation, market power, Certificate-of-Need law, reimbursement, and margin pressure.
3. Run the 3-role editorial gate (citation, scope incl. scale, numeracy).
4. Promote only the bounded claim; keep clinical-study status, facility-licensing/accreditation status, payer/coverage determination, medical advice, CON determination, and endorsement out of scope.

Failure or degraded path: if evidence, review, affordability, payer/market framing, transfer-suitability, or scope is insufficient, the claim stays held or downgraded with a next evidence step.

Outputs: promoted, held, or downgraded claim plus a review record.

Handoffs: review steward to maintainer or design owner.

Validation evidence: review record, editorial-gate result, future `EVID-*`.

### OPS-005: Apply VTRACE One Deliverable At A Time (serves all NEEDs)

Trigger: a maintainer asks to advance SHIELD's VTRACE baseline.

Inputs: existing `docs/vtrace/` artifacts, `.roles`, VTRACE templates, the active wave ledger.

Normal path:

1. Create or revise exactly one VTRACE deliverable.
2. Use prior VTRACE IDs as parent IDs.
3. Review against the relevant `.roles` subset to a fixed point.
4. Run doc validation; record the stage in the wave ledger.
5. Keep child-repo artifacts scoped; do not mix with TRACKER pointer updates.

Failure or degraded path: if repo state is dirty or out of scope, keep edits scoped to the one deliverable and report status.

Outputs: one reviewed VTRACE artifact with stable IDs.

Handoffs: maintainer to next-stage author.

Validation evidence: doc QA, role review notes, future implementation evidence.

### OPS-006: Classify Tier And Check SLA Conformance (serves NEED-007, NEED-002)

Trigger: a network is added or re-evaluated, or a market's adequacy is assessed.

Inputs: element attributes (facility role, service breadth, staffed beds/providers/slots, access time, outcomes), the T1â€“T4 tier model, and the per-tier SLA contract.

Normal path:

1. Classify the element into T1 (Quaternary/Academic Medical Center), T2 (Tertiary/Regional Hospital), T3 (Community Hospital/Secondary), or T4 (Primary Care/Clinic).
2. Look up the tier's SLA (access time, capacity, service breadth, outcomes).
3. Assess conformance: does the element meet its tier SLA, with the demand basis named (Surge vs Baseline, REQ-007)?
4. Record a tier-SLA gap where the element under-serves its tier promise.

Failure or degraded path: if tier, SLA, payer/coverage, workforce, referral, CON, or margin inputs are missing, the element is held with a source-needed label rather than assigned a tier or upgrade assumption on faith.

Outputs: a tier label, an SLA-conformance assessment, and any tier-SLA gap.

Handoffs: maintainer to gap author and review steward.

Validation evidence: tier/SLA record, conformance check, future `EVID-*`.

### OPS-007: Run Analysis At A Chosen Scale (serves NEED-008)

Trigger: a maintainer scopes a corpus, gap, or design run to a scale.

Inputs: the scored corpus tagged by scale (international/national/regional/local), and the market/jurisdiction filter.

Normal path:

1. Select the scale and market/jurisdiction for the run.
2. Filter the corpus to elements at (or explicitly relevant to) that scale.
3. Score, tier, gap, or design strictly within the scale; do not aggregate across scales.
4. If a cross-scale relationship matters (e.g. a local clinic shortage affecting a regional referral-region result), record it as an explicit, labelled cross-scale note (CON-007).

Failure or degraded path: if elements lack a scale tag, they are excluded and flagged, not silently mixed in.

Outputs: a scale-scoped corpus/gap/design view with any cross-scale notes.

Handoffs: maintainer to analyst and review steward.

Validation evidence: scale-filtered artifact, future `EVID-*`.

## Operational Assumptions

- SHIELD is greenfield and code-free: VTRACE is authored ahead of implementation, so scenarios describe intended operation, not retrofit.
- The active VTRACE sequence is MISSION â†’ CONOPS â†’ REQUIREMENTS â†’ SPECIFICATION_BASELINE before implementation planning.
- Data sources may be lagged, aggregated, censored, payer-skewed, proxy-only, surge-biased, or baseline-biased; SHIELD records intended acquisition and validation even when a full pass is deferred.
- `.roles` review is part of SHIELD operations and must change evidence posture, claim labels, transfer findings, or next steps when it finds a gap.
- TRACKER remains the portfolio snapshot repo; SHIELD owns repo-local implementation and VTRACE artifacts.

## Role Review Notes

| Role Lens | CONOPS Impact | Disposition |
|---|---|---|
| Scope Keeper | Scenarios describe workflows; no specific network/gap/design prescribed; OPS-007 enforces scale scoping. | pass |
| Citation Auditor | No new quantitative claims; scenarios name repo-local artifacts and future evidence paths. | pass |
| Numeracy Checker | No arithmetic, units, score ranges, capacity, outcomes, wait-time, or cost figures. | pass |
| Health-System Planner / Operations Officer | Scoring and review scenarios require demand-basis evidence before promotion and do not assume average utilization proves surge adequacy. | pass |
| Payer & Consolidation Realist | OPS-004 explicitly requires the realist to challenge payer mix, consolidation, market power, CON law, reimbursement, and margin pressure. | pass |
| Data steward (added lens) | Initial draft underspecified source-label, payer/market, and scale custody; resolved by adding the Data steward actor and the source-needed/scale hold path in OPS-001. | resolved |

Fixed-point note: one actionable finding (source-label/scale/payer-market custody under-specified) was raised and applied. No unresolved critical or major finding remains.

## Open Questions

| ID | Question | Disposition |
|---|---|---|
| OQ-001 | What is the exact dimension pool and its definitions? | Defer to `REQUIREMENTS.md` and `SPECIFICATION_BASELINE.md`. |
| OQ-002 | Which data sources become the first `EVID-*` sources, and at what cadence? | Defer to `data/sources.md` and `VERIFICATION.md`. |
| OQ-003 | What demand basis (peak/surge vs average/baseline) anchors capacity/adequacy scoring? | Defer to `SPECIFICATION_BASELINE.md`. |
| OQ-004 | How is the `scale` tag represented and enforced in the corpus schema? | Defer to `SPECIFICATION_BASELINE.md` / `INTERFACES.md`. |
| OQ-005 | Which physical-lifeline metrics fail or need redefinition for referral/catchment pathways? | Defer to `SPECIFICATION_BASELINE.md`, `REVIEW.md`, and the first calibration wave. |

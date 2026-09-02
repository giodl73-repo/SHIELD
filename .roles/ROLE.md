# SHIELD — Role Index

Four tiers of review roles. Read this before opening any role file. Reviews of corpus entries, gap findings, design proposals, tier/SLA definitions, and VTRACE deliverables run against these files and record dispositions (`pass` / `finding` / `defer`).

---

## Parliament roles (7 voices)

Adversarial expert voices. They plant incompatible stakes; the argument record is the output, not consensus. No voice is skipped. A good project survives all seven; a weak one collapses under one or two, and the collapse is the finding.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/health-system-planner.md` | Health-System Planner | System access + resilience vs. single-facility framing |
| `parliament/clinician-care-delivery.md` | Clinician / Care-Delivery Lead | Deliverable, safe, staffed care vs. brochure-capacity counts |
| `parliament/operations-capacity-officer.md` | Operations & Capacity Officer | Surge/peak demand + diversion vs. average-census optimism |
| `parliament/health-economist.md` | Health Economist | Benefit-cost + population value vs. discretionary-service-line inflation |
| `parliament/equity-access-advocate.md` | Equity & Access Advocate | Underserved/shortage-area coverage + disparities vs. aggregate-average benefit |
| `parliament/public-health-advocate.md` | Public-Health & Prevention Advocate | Population outcomes + prevention vs. acute-bed-count expansion |
| `parliament/payer-consolidation-realist.md` | Payer & Consolidation Realist | Payer-mix/consolidation/CON/margin constraints vs. assumed-free capacity |

---

## Editorial roles (3 voices)

Form gate before `validated` status. Run after parliament, not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every quantity sourced in `data/sources.md` or labelled |
| `editorial/scope-keeper.md` | Scope Keeper | Artifact stays within its declared type, **scale**, schema, pool, and tier model |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent (beds·1,000-pop⁻¹ / providers·100k-pop⁻¹ / minutes / % / $); magnitudes sane; arithmetic and 0–10 scale clean |

---

## Stakeholder roles (cross-cutting lenses)

Not reviewers — lenses for who the network serves, used during corpus scoring, gap analysis, and tier/SLA assignment.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/patient-household.md` | Patient / Household | Timely, affordable, nearby care |
| `stakeholders/frontline-clinician.md` | Frontline Clinician | Workload, safety, ability to deliver care |
| `stakeholders/rural-underserved-community.md` | Rural / Underserved Community | Having any access at all; closures |
| `stakeholders/public-health-agency.md` | Public-Health Agency | Population outcomes, prevention, surge readiness |
| `stakeholders/payer-purchaser.md` | Payer / Purchaser | Cost, coverage, value of spend |

---

## Panel reviewer roles (illustrative peer panel)

Archetype academic/practitioner peer reviewers for SHIELD research outputs. See `panel-reviewer/panel.md`. Used for paper-grade methodology review, distinct from parliament and editorial.

---

## PITFALL gates

| Pitfall | Gate | Required roles |
|---|---|---|
| `SHIELD-PF-05` | Public access claim boundary. Blocks aggregate CMS, USDA, and HRSA baselines from promotion as an access model, shortage map, staffing finding, adequacy finding, intervention target, savings claim, HLT admission, clinical/facility/payer action, release-ready product, or endorsement until command, source grain, scale, demand basis, held claims, allowed/blocked claims, role dispositions, and downstream owner acceptance are visible. | Health-System Planner; Clinician / Care-Delivery Lead; Operations & Capacity Officer; Health Economist; Equity & Access Advocate; Public-Health & Prevention Advocate; Payer & Consolidation Realist; Citation Auditor; Scope Keeper; Numeracy Checker |

---

## How reviews are recorded

When a `docs/vtrace/` deliverable, corpus entry, gap finding, design proposal, or tier/SLA definition is being settled, the relevant subset of this panel is applied and dispositions are recorded in:

- the deliverable's **Role Review Notes** section, and
- the active wave pulse ledger.

A stage reaches its **fixed point** when no unresolved critical or major actionable finding remains and every deferred item names a later stage or work package.

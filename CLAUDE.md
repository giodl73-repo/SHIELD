# SHIELD — House Rules

## 1. Project Identity

SHIELD is a **research and conceptual-design project for Healthcare Access 2.0** — a data-driven upgrade plan for the healthcare delivery network (hospitals, emergency departments, clinics, specialty centers, providers, and referral/transfer/care pathways), applicable at international, national, regional, and local scales. The mission: score an existing network against a calibrated dimension pool, find the gaps (travel-time and appointment access, bed/provider/slot capacity, workforce shortage, referral discontinuity, service-line gaps, affordability/coverage barriers, surge fragility, tier-SLA shortfalls), and design into them.

**The architectural bet** — borrowed from ROUTE/PYLON/GAUGE/BASIN/PACKET/TARMAC/HARBOR/DRAIN: score enough of an existing network on enough dimensions and the design space tells you its own structure. SHIELD deliberately stress-tests that bet on a service-and-human system rather than a physical lifeline. The gaps aren't invented; they're found. A project designed into a real gap is better evidence than one invented from first principles.

**The testable hypothesis**: there is a set of ≤20 interventions — at a stated scale — that, if built or adopted to Healthcare 2.0 standards, would shorten time-to-care, close provider-shortage and coverage gaps, strengthen referral continuity, and harden surge resilience. **A rigorous null result is as valid as a positive one.** A dimension that does not transfer cleanly from physical lifelines is reported as a finding, not forced into the model.

Sibling projects: **ROUTE** (highways), **PYLON** (grid), **GAUGE** (rail), **BASIN** (water), **PACKET** (internet), **TARMAC** (air), **HARBOR** (ports), and **DRAIN** (wastewater). SHIELD borrows their structural patterns; SHIELD's own rules apply here.

## 2. Multi-Scale Rule

Every corpus element carries a **scale** (`international` / `national` / `regional` / `local`) and a market/jurisdiction. Scores, tiers, gaps, and design proposals are interpreted **within their stated scale**. A claim must not compare or aggregate across scales without saying so. The same dimension pool and tier model apply at every scale; only the scope of the run changes.

## 3. The Pipeline

```
CORPUS (score existing networks) → RUBRIC CALIBRATES → GAP MAP
  → CONCEPT → SCORE → PARLIAMENT → DESIGN → HANDOFF
```

**Anchor rule**: one existing element must go through the full pipeline (corpus entry → calibration pass → gap-map entry) before any proposed project is analyzed. One proposed project must survive parliament manually before any skill is built. YAGNI is the law.

## 4. Quality Bar

- Research-paper-level estimates. Order-of-magnitude beds, provider supply, appointment availability, travel time, transfer/referral continuity, coverage, quality/outcome, and cost figures with citations.
- Every number cited. An uncited number blocks promotion to `validated`.
- No capacity or adequacy claim dressed as solved planning — conceptual analysis only, with evidence labels and the demand basis named (`Surge` vs `Baseline`).
- No hand-waving on economics. Marginal or negative benefit-cost projects, payer-mix constraints, and market-power constraints are reported as such.
- Data sources declared. Every corpus entry names its source (`data/sources.md`).

## 5. Forbidden Vocabulary

In corpus entries and design proposals: no "obviously needed," "critical gap," "long overdue," or any pre-judged framing before the score supports it. Claims must cite (a) dimension, (b) score, (c) corpus comparison, (d) scale. "This HRR scores 8.4 on Access vs. a corpus mean of 5.1 at regional scale" beats "this is a critical shortage."

## 6. VTRACE Governance

SHIELD's planning baseline lives in `docs/vtrace/` and is authored one deliverable at a time to a `.roles` review fixed point. Do not start implementation code until the relevant work package is accepted. implementation automation builds later code from work packages; this foundation is markdown-only.

## 7. Review Panel

Seven adversarial parliament voices and a three-role editorial gate review every promotable artifact. See `.roles/ROLE.md`. No voice is skipped. The payer-and-consolidation realist exists because reimbursement, payer mix, hospital and physician-practice consolidation, Certificate-of-Need law, and margin pressure govern what care actually gets built, kept open, or closed — that market tension is a feature, not an accident.

## 8. Portfolio Discipline

SHIELD implementation changes belong in this repo. TRACKER receives only intentional submodule pointer updates after intake. Do not make build or validation correctness depend on TRACKER-relative paths.

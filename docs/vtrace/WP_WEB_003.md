# WP-WEB-003: Frozen designation-component geography browser

Status: accepted for implementation, 2026-10-29.
Parents: WP-010, WP-WEB-001/002. National frozen aggregate exploration only.

## Contract

Expose unchanged load_hrsa_geography_fixture, hrsa_geography_baseline_json and
hrsa_geography_held_pack_json in a new geography view. Reuse the frozen July 31
Designated cohort: 20,368 component rows / 7,682 IDs; 15,524 area rows / 2,838
area IDs; 4,844 facility rows / IDs. No other status, jurisdiction, individual
county, map, hospital identity or population grain is available.

Optional geography_cohort selects Area designations, Facility designations or
one exact area component class (Census Tract, County Subdivision, Single County).
Null/absent selects all Designated geography. Reject unknown cohorts and any
non-null status/jurisdiction. Reject geography_cohort on all other views. Keep
the strict unknown-field and 64 KiB boundary. Selected counts are source fields,
not JS arithmetic; old seven canonical results remain byte-identical.

Show selected component rows, designation IDs and distinct recorded county
keys with their units. County keys are not mutually exclusive across cohorts
or classes: 2,616 area / 1,879 facility keys overlap within the 2,932-key union.
Never add them or invent per-county coverage, area/population, overlap size or
hospital assignments. Preserve 2,076 single-/762 multi-component area IDs and
2,683 single-/155 multi-county area IDs as separate partitions. Single County
is a component class; it includes 89 multi-component/multi-county IDs. Keep
three area classes separate from facility/unknown component grain. A structurally
valid five-digit common county key matching state prefix is not an independent
check against a county-boundary master or evidence of whole-county shortage.

Always show the fixed Designated county-key check: 20,350 valid rows / 7,664 IDs;
18 residual rows / IDs, split 17 placeholders and one state-prefix inconsistency.
All 18 residuals are facility records; area residual rows are zero. No repair or
allocation. Class-specific residual is unobserved (null) in the projection.
Baseline canonical bytes match the original CLI. Preserve all native validators,
fixtures, held pack and research status. Nine service floors remain null; CMS
facility join and admission remain false. All clinical/designation/workforce/
facility/payer/funding/fiscal authority remains false. No provider/FTE formula
view, service finding, cost, effect or research-candidate promotion.

Publish exact aggregate geography-source.json, SHA custody and source note.
Historical 48,303,351-byte CSV hash is context only: original bytes are not
bundled, reacquired or reverified here; mutable publisher URL may return a newer
release. No HRSA request on page load, uploads or personal inputs. UI has a
cohort selector enabled only for geography; fixed-context tables stay labelled
when selection changes. Loading/error, stale-result clearing, reset, downloads,
keyboard/mobile and existing views persist. Artifact <5 MB.

TAXLANE health/Medicare evidence wording and assertions only; no ledger, rates,
admission, controls or dependency changes. TRACKER records the release after
child PR/CI/deploy gates, exact clean defaults, live artifacts and navigation.
Other satellites, ICELINES, BISECT and unrelated owner state are no-op.

## Exit and role review

Pinned fmt/check, locked workspace/all-target tests, strict native/WASM Clippy.
Six geography fixtures plus seven old actual-WASM/native canonical and original
baseline comparisons; seven pre-change canonical byte regressions. Browser
cases cover cohorts, all residual and non-additive context, holds, rejection,
reset/download/source hashes, keyboard/mobile and TAXLANE health/Medicare routes.

Implementing-agent review (not independent clinical or publisher endorsement):

- Health-System Planner: pass; national structure is not local access. Defer
  patient-relevant access to PRODUCT_PLAN's compatible provider/service work.
- Clinician/Care-Delivery Lead: pass; no staffing, safety or clinical result.
  Defer staffed services and outcomes to a bounded candidate.
- Operations/Capacity Officer: pass; no Baseline/Surge or transfer SLA. Defer
  operating evidence to a bounded service candidate.
- Health Economist: pass; no population denominator, costs or savings. Defer
  economics/incidence to candidate review.
- Equity/Access Advocate: resolved; tract/subdivision and multi-county grain
  remain explicit; no whole-county/people coverage. Defer disparities/access.
- Public-Health/Prevention Advocate: pass; no deduplicated people or outcomes.
  Defer population/health evidence to matched service work.
- Payer/Consolidation Realist: pass; no payer, ownership or funding decision.
  Defer feasibility evidence to candidate review.
- Citation Auditor: resolved; exact derivative custody is separate from absent
  historical primary bytes and mutable URL. No new source verification claim.
- Scope Keeper: resolved; only Designated national cohorts, no per-county map
  or CMS identity join. Research public-release hold does not forbid the
  owner-authorized aggregate browser; candidate authority stays held.
- Numeracy Checker: resolved; rows, IDs and overlapping county-key counts have
  separate units; fixed totals/residuals persist; unobserved stays null.

Patient/household, clinician, rural community, public-health and payer lenses
retain separate access, workload, local service, outcome and affordability
unknowns with those deferrals. No unresolved critical/major actionable finding
remains. Repeat all ten lenses at exit. Rollback: revert adapter/evidence changes
and redeploy the prior artifacts, preserving release records.

## Implementation exit: 2026-10-29

Implemented six geography queries and cohort controls, fixed Designated area/
facility/class tables, separate multiplicities and unrepaired residual.
Exact aggregate/source-note downloads preserve historical-primary limits.
No source crate, derived aggregate, validator, dependency or research status
changed. All seven pre-change canonical results remain byte-identical.

Local validation: pinned fmt/check, locked workspace/all-target tests (64),
strict native/WASM Clippy; fourteen SHIELD and five TAXLANE browser cases pass.
Thirteen actual-WASM/native canonical and original baseline comparisons pass.
Desktop/mobile screenshots inspected; wide tables support keyboard scrolling
without page overflow. Local SHIELD bundle is 621,974 bytes before clean release.

All ten implementing-agent lenses repeated: Health-System Planner, Clinician/
Care-Delivery Lead, Operations/Capacity Officer, Health Economist, Equity/Access
Advocate, Public-Health/Prevention Advocate, Payer/Consolidation Realist, Citation
Auditor, Scope Keeper and Numeracy Checker pass within the accepted contract.
Preimplementation deferrals remain in PRODUCT_PLAN's matched service/candidate
work. No unresolved critical/major actionable finding or independent endorsement
is claimed. Published exact defaults and live custody/navigation evidence are
recorded by TRACKER after deployment.

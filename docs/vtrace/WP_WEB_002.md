# WP-WEB-002 — Frozen primary-care shortage-registry browser queries

Status: accepted for implementation, 2026-10-28.
Parents: WP-WEB-001, the existing native HRSA primary-care census/held-pack
contracts and aggregate-only source/citation/scale/privacy rules.

## Reader task and source contract

Add a registry view to the existing SHIELD source explorer. Reuse unchanged
`shield_cms_access::load_hrsa_primary_care_fixture`, native baseline serializer
and held-pack serializer. Query all frozen status rows or one known status.
Keep component rows distinct from unique HPSA IDs within a status. The all-status
projection has no deduplicated designation total. Do not sum ID counts across
statuses, assign hospital identities or invent status-by-type/rurality/state
cross-tabs. Designated-type and rural-status summaries always describe only the
Designated cohort, independently of the selected status. The rural summary's
multi-status bucket is not another rural class; retain its multiplicity label.

Use the committed July 31, 2026 aggregate exactly. Show the June 30 quarterly
summary separately with its July 1 data-as-of date, 9,003 reported designations
and explicit false cross-vintage reconciliation readiness. Never manufacture a
delta, trend, percentage reconciliation or common denominator across the files.
The native field name `current_registry` means the frozen source, not current
October 2026. Label every view with its own dates and source units.

Publish the exact aggregate fixture and a source note/custody hash. Original
48,303,351-byte CSV and quarterly PDF are not in this portable bundle and are
not reacquired by this work. Source URLs may now return newer releases; original
hashes describe prior recorded captures, not newly verified downloads. Aggregate
fixture custody must not be called primary-source byte verification. No source
count, data vintage, validator, baseline, held pack or research status changes.

## Adapter and interface

Add `registry` to View and an optional status field to the strict 64 KiB query.
Registry requires jurisdiction null and a known source status or null (all rows).
A status filter is rejected on CMS views. Preserve all existing CMS query result
bytes for legacy fixtures. The new projection reports selected component rows
and within-status IDs or null, with absent hospital/person/access/capacity and
cross-tab observations. Native baseline and held-pack canonical bytes must match
CLI serializers. No registry arithmetic or reconciliation is copied into JS.

UI: status selector enabled only for registry; jurisdiction only for footprint.
Render selected status counts, all source status rows, fixed Designated type and
rural multiplicity summaries, component multiplicity and separately dated
quarterly type/rurality partitions. Make selected versus fixed cohort labels
explicit. Source hashes, original mutable links, aggregate/JSON/WASM downloads,
loading/error, stale-result clearing, reset and keyboard/mobile access persist.
All clinical, designation, workforce, facility, payer, funding and fiscal holds
remain; the held pack's public-release flag concerns research-candidate authority,
not the owner-authorized aggregate browser publication.

TAXLANE updates health and Medicare provider/caveat wording and assertions only;
no accounting, ledger, rates, candidates or admission change. TRACKER coordinates
and snapshots after child publication. All other satellites, ICELINES and BISECT
are no-op in this package. No new runtime dependency or product coupling.

## Exit

Pinned Rust fmt, locked workspace/all-target check/tests, strict native/WASM
Clippy; typed boundary tests for all/known/unknown status, unsupported grains and
all held observations. Actual-WASM/native canonical parity for existing CMS
fixtures plus all four registry queries; exact original native baseline bytes.
Add browser cases for status switching, cohort labels, multiplicities, separate
vintages, null/non-additivity, unsupported queries, reset/download, keyboard/
mobile, source paths/hashes and health/Medicare navigation. Retain prior cases.
Artifact <5 MB. PR gates precede master-only deployment. Clean exact defaults,
source artifact/live hashes, all browser cases and TAXLANE roundtrips precede the
scoped root snapshot; preserve unrelated owner state.

## Role review notes — preimplementation fixed point

Implementing-agent lenses, not independent clinical or publisher endorsement:

- Health-System Planner: pass; a national registry observation is not a care-
  delivery/access finding. Defer matched service evidence to PRODUCT_PLAN's
  compatible provider/site and staffed-service work.
- Clinician/Care-Delivery Lead: pass; no staffing, treatment, quality/safety or
  actual access inference. Defer clinical/staffed-service evidence to that work.
- Operations/Capacity Officer: pass; no Baseline/Surge, wait, transfer or response
  SLA inference. Defer operating evidence to a bounded source-backed candidate.
- Health Economist: pass; no per-capita need, costs, savings or financial result.
  Defer costs/incidence to a bounded source-backed candidate.
- Equity/Access Advocate: resolved: preserve rural-status multiplicity and unknowns;
  no county-to-patient or designation-to-hospital inference. Defer patient access/
  disparities to matched service evidence.
- Public-Health/Prevention Advocate: pass; no deduplicated population or outcome
  inference. Defer prevention/outcome evidence to a bounded sourced candidate.
- Payer/Consolidation Realist: pass; no coverage, payment, CON, ownership or margin
  decision. Defer payer/service-feasibility evidence to candidate review.
- Citation Auditor: resolved: exact aggregate custody, recorded historical raw
  hashes, unavailable bundled originals and mutable current links distinguished.
- Scope Keeper: resolved: no registry jurisdiction or status-by-type/rurality
  cross-tab; fixed Designated cohorts and different vintages labelled separately.
- Numeracy Checker: resolved: rows and within-status IDs have separate units;
  no all-status unique-ID total, no cross-vintage reconciliation, null stays null.

Patient/household, frontline clinician, rural community, public-health agency and
payer lenses retain their separate access/workload/shortage/outcome/affordability
unknowns with those deferrals. No unresolved critical/major actionable finding
remains in this contract. Repeat all ten lenses at exit. Rollback: revert the
adapter/view and redeploy the prior artifact; retain source and release evidence.

## Implementation exit — 2026-10-28

Implemented the registry adapter, four registry queries, fixed Designated
partitions, separate quarterly tables, exact aggregate download and explicit
historical-primary custody limits. Existing three CMS canonical results are
byte-identical to the pre-change capture. No native source crate, aggregate,
source value, dependency, held pack or validator changed.

Local exit: pinned Rust fmt/check, locked workspace/all-target tests (61),
strict native and wasm32 Clippy; seven actual-WASM/native canonical and native
baseline comparisons; ten browser cases pass. TAXLANE's five browser cases pass.
Artifact is 525,053 bytes at this local build; release custody and live checks
are recorded separately in the portfolio wave after deployment.

Repeated all ten implementing-agent lenses: Health-System Planner, Clinician/
Care-Delivery Lead, Operations/Capacity Officer, Health Economist, Equity/Access
Advocate, Public-Health/Prevention Advocate, Payer/Consolidation Realist, Citation
Auditor, Scope Keeper and Numeracy Checker. Each passes within the accepted
contract and retains the preimplementation deferrals. Registry rows and IDs,
multiplicity and null union, distinct vintages and aggregate-versus-primary
custody are explicit. No unresolved critical/major actionable finding remains;
this is an implementing-agent review, not independent clinical endorsement.

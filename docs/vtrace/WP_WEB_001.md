# WP-WEB-001 — Frozen hospital-footprint and rurality explorer

Status: accepted for implementation automation, 2026-10-23. Owner objective:
deliver all sixteen TAXLANE satellites, including bounded software publication.
The implementing coding agent executes this accepted package and repo CI; the
completed implementation baseline is not reopened as a markdown-only foundation.

## Reader task

Query the existing CMS May 13, 2026 hospital-footprint release by jurisdiction,
inspect independent national hospital-type/emergency-flag partitions, and switch
to the existing CMS–USDA RUCC 2023 county-rurality join. Keep all 72 unmatched
facilities visible and unallocated. Use unchanged shield-cms-access loaders,
validators, baseline serializers and held-pack functions. Add only a bounded
query/projection adapter; no source editing or new cross-source identity join.

Jurisdiction counts are a source subset; type/emergency partitions remain
national because no state/type cross-tab is established here. Rurality is a
national county-class distribution, not a state query or patient classification.
Reject unsupported cross-tabs and unknown jurisdiction keys. The two source
views retain their own grains and original custody. They do not establish
travel, staffed beds, clinicians, service availability, need, quality, outcomes,
affordability, equity, adequacy, surge resilience, cost, or savings. Preserve
existing held/null service floors and all false authority fields.

This task is source exploration, not medical guidance, provider ranking,
individual care, patient data, facility action, licensing, accreditation, payer,
coverage, fiscal admission or tax-rate policy. Fixed historical releases are
not live CMS/USDA feeds. Local worker computation; no upload/storage or personal
inputs. Editable inputs are typed view/jurisdiction queries, not editable source
counts or hypothetical clinical recommendations.

## Implementation and exit

Add standalone shield-web native replay and worker WASM adapter, strict typed
JSON/unknown-field rejection and <=64 KiB. Query two existing source views;
selected-jurisdiction counts/share are computed in Rust against the native
validated national denominator. No service arithmetic or source rules in JS.
Paths: manifests/toolchain/lock, crates/shield-web, query fixtures, web,
build/CI tooling, locked browser packages/tests, README, this contract and wave.

Exit: workspace format/check/clippy/tests; native/executing-WASM canonical byte
parity for national, selected-jurisdiction and rurality queries; original native
baseline bytes retained for parity; browser view/jurisdiction controls,
independent partitions, 72-row residual, preserved null/held authority, unsupported
query errors, stale-result clearing, reset/download, loading/failure,
keyboard/mobile/prefix; <5 MB artifact with source fixtures/custody, WASM,
licenses and exact-commit provenance. PR checks precede master-only deploy;
verify live artifact and TAXLANE health navigation before pointer snapshot.

## Role Review Notes

Implementing-agent lenses, no independent endorsement. All seven Parliament
voices applied before editorial:

- Health-system planner: pass for source denominator exploration, not access.
- Clinician/care-delivery lead: pass with staffed care/quality/safety held/null.
- Operations/capacity officer: pass without Baseline/Surge capacity inference;
  defer such claims to PRODUCT_PLAN's matched staffed-service evidence work.
- Health economist: pass with no per-capita need, value, cost or savings claim;
  defer economics to that matched candidate/evidence work.
- Equity/access advocate: pass with unmatched residual and county/patient
  distinction; defer patient-relevant access/disparities to that evidence work.
- Public-health/prevention advocate: pass without population outcome inference;
  defer outcomes to that evidence work.
- Payer/consolidation realist: pass with no coverage, CON, ownership or margin
  premise; defer payer/service-feasibility evidence to that candidate work.
- Citation auditor: pass with exact frozen custody and included source fixtures.
- Scope keeper: pass for declared national partitions/source-subset query, no
  unsupported state-by-type or state-by-rurality inference.
- Numeracy checker: pass with facility counts and native denominators distinct
  from beds, patients, FTE, population and clinical demand.

Patient/household, clinician, rural-community, public-health and payer lenses
retain separate access, workload, shortage, outcome and affordability unknowns;
defer real service conclusions to the named matched-evidence/candidate work.
No critical/major actionable finding remains in this contract. Repeat review
at implementation exit; advance no existing clinical/fiscal hold. Rollback:
revert source explorer and previous Pages artifact. Child commit/push precedes
scoped TRACKER pointer update.

## Implementation exit

2026-10-23: 58 native workspace tests pass; workspace formatting, check and strict
Clippy pass. Six actual-WASM browser cases pass, including canonical/native-byte
parity for all three queries, independent source subset checks, rurality residual,
unsupported queries, loading/failure, keyboard/mobile and source-custody routes.
Desktop/mobile screenshots inspected. Artifact is approximately 426 KB locally.
All existing source-core libraries remain unchanged. Implementing-agent role and
stakeholder lenses repeated against the implementation: no critical/major
actionable finding; named evidence defers above and all clinical/fiscal holds
remain. Hosted checks gate master-only publication. TRACKER records exact live
artifact identity and TAXLANE navigation before the pointer snapshot.

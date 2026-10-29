# WP-WEB-004: Frozen designation-formula capacity browser

Status: accepted for implementation, 2026-10-29. Parents: WP-011, WP-WEB-001..003.
National frozen Designated aggregate exploration, not a staffed-care model.

## Contract

Reuse unchanged load_hrsa_capacity_fixture, hrsa_capacity_baseline_json and
hrsa_capacity_held_pack_json. Null/absent capacity_cohort selects all formula-
bearing designations (3,388); known choices are Area designations (2,838),
Correctional designations (550), Policy-excluded facilities (4,294). Exclusions
are missing by policy, not zeros; the full 7,682 Designated IDs remain the fixed
coverage denominator. Reject other cohorts, non-null status/jurisdiction/
geography_cohort and unknown fields. capacity_cohort is rejected on other views.
Retain the 64 KiB input bound and all thirteen prior canonical result bytes.

Rust returns selected ID count, exact recorded FTE/shortage ten-thousandths and
native need-met basis points, with exact display strings (four FTE decimals,
two percent decimals). No floating-point or formula arithmetic in JS. All-
formula, area and correctional values come from existing native source fields;
all-formula ratio is not an average of cohort percentages. Excluded FTE, shortage
and ratio are null. Do not combine formula-excluded populations or designation
values into unique physicians, patients or counties. NP/PA supply is excluded.

Keep fixed formula and exclusion tables visible when selection changes. Show
five excluded types, FTE zero/positive and shortage zero/positive coverage,
recorded provider-ratio labels (including 1000:0 without inventing a quotient),
area versus correctional formula populations separately, and exact area
population identity: designation minus served minus underserved = -1 person-
unit rounding residual across one ID (2,837 exact IDs). Both native tolerance
checks cover all 2,838 area IDs (half-person served, 0.01-FTE shortage). Retain
4,284 excluded IDs with population fields and ten missing without treating
missing fields as zero or making those populations unique people.

No workforce census, staffed hospital/service-line supply, patient access,
adequacy, clinical/payer/funding/fiscal action, cost, effect or candidate
promotion. Nine service floors remain null; unique physician supply, CMS
facility capacity and TAXLANE admission stay false. Original native baseline
canonical bytes match the CLI; all source validators/data/held packs remain.
Native current wording means frozen July 31, not current October data.

Publish exact capacity-source.json with aggregate SHA and source note. Historical
CSV (48,303,351 bytes) and metadata XLSX (27,297 bytes) hashes are recorded
context only: original files are not bundled, reacquired or reverified here.
Mutable publisher endpoints may return newer releases. No HRSA request on load,
no uploads or personal inputs; existing source attribution/terms persist.

UI cohort selector is enabled only in capacity view. Loading/error, stale-result
clearing, reset, exact downloads, keyboard/mobile and existing views persist.
Artifact <5 MB. TAXLANE changes health/Medicare evidence wording/assertions only;
no accounting, ledger, rates, admission, controls or product dependencies.
TRACKER snapshots after child PR/CI/deployment gates, clean exact default SHAs,
artifact/live hashes and health/Medicare keyboard routes. All other satellites,
ICELINES, BISECT and unrelated owner state are no-op.

## Exit and role review

Pinned fmt/check, locked workspace/all-target tests, strict native/WASM Clippy.
Four capacity fixtures plus thirteen existing actual-WASM/native canonical and
original baseline comparisons; thirteen pre-change canonical byte regressions.
Browser tests cover cohorts, exclusion/null-versus-zero, exact decimals,
non-additivity, fixed population residual/tolerances, holds, rejection, reset,
source downloads/hashes, keyboard/mobile and health/Medicare navigation.

Implementing-agent lenses, not independent clinical/publisher endorsement:

- Health-System Planner: pass; designation formulas are not local service
  supply. Defer matched patient-relevant service work to PRODUCT_PLAN.
- Clinician/Care-Delivery Lead: resolved; physician FTE does not establish
  staffed service lines; NP/PA omitted. Defer staffing/safety to matched sources.
- Operations/Capacity Officer: pass; no Baseline/Surge, wait or transfer SLA.
  Defer operating evidence to a bounded service candidate.
- Health Economist: pass; no unique workforce/population, costs or savings.
  Defer incidence/economics to candidate review.
- Equity/Access Advocate: resolved; no zero-filling excluded facility classes
  or claiming percent access. Defer disparities/access to matched service work.
- Public-Health/Prevention Advocate: pass; formula population sums may overlap,
  no population outcome or need satisfaction. Defer population/health evidence.
- Payer/Consolidation Realist: pass; no eligibility, payment, coverage, ownership
  or funding action. Defer payer/feasibility evidence to candidate review.
- Citation Auditor: resolved; aggregate custody differs from absent original
  CSV/XLSX bytes and mutable links. No new source verification claim.
- Scope Keeper: resolved; four national Designated cohorts only; no status,
  geography or CMS join. Research public-release hold remains distinct from
  owner-authorized aggregate browser publication.
- Numeracy Checker: resolved; integer e4/bps retained, Rust display strings,
  excluded null distinct from observed zeros, signed -1 residual retained;
  no averaging cohort ratios or deduplicated workforce/population inference.

Patient/household, frontline clinician, rural community, public-health and payer
lenses retain access, workload, local service, outcome and affordability unknowns
with those deferrals. No unresolved critical/major actionable finding remains.
Repeat all ten lenses at exit. Rollback: revert adapter/evidence changes,
redeploy prior artifacts and retain release records.

## Implementation exit: 2026-10-29

Four formula/exclusion queries use unchanged native source fields, exact
integer e4/bps and Rust display strings. Fixed formula, exclusion, observed
zero/presence, ratio-label and population-residual/tolerance tables remain
visible independently of selection. Policy-excluded quantities remain null.
No source crate, derived aggregate, validator, dependency or research change.

Local exit: pinned fmt/check, locked workspace/all-target tests (67), strict
native/WASM Clippy; eighteen SHIELD and five TAXLANE browser cases pass.
Seventeen actual-WASM/native canonical and original baseline comparisons and
thirteen pre-change canonical byte regressions pass. Desktop/mobile screenshots
inspected; keyboard table scrolling without page overflow. Local SHIELD bundle
is 723,741 bytes before clean release; published custody recorded separately.

All ten implementing-agent lenses repeated: Health-System Planner, Clinician/
Care-Delivery Lead, Operations/Capacity Officer, Health Economist, Equity/Access
Advocate, Public-Health/Prevention Advocate, Payer/Consolidation Realist, Citation
Auditor, Scope Keeper and Numeracy Checker pass within the accepted contract.
Preimplementation deferrals remain in PRODUCT_PLAN's matched service/candidate
work. No unresolved critical/major actionable finding or independent endorsement
is claimed. TRACKER records exact defaults, workflows, artifacts and live checks
after child publication.

# SHIELD

**Healthcare Access 2.0 — multi-scale care-delivery network analysis.**

**A bed is not access if the patient cannot reach, enter, afford, or continue through care.**

SHIELD scores facilities, providers, service lines, and referral or transfer
pathways across access, capacity, quality, workforce, affordability,
continuity, equity, and surge resilience.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

## Infrastructure 2.0 family

SHIELD is one domain implementation of a shared evidence-first method:

```text
PUBLIC SOURCES → CORPUS → SCORE → SERVICE PROMISE → GAP MAP
                                                     ↓
                                      CONCEPT → REVIEW → DESIGN
```

| Lane | Repositories |
|------|--------------|
| Movement | [ROUTE](https://github.com/giodl73-repo/ROUTE), [GAUGE](https://github.com/giodl73-repo/GAUGE), [TARMAC](https://github.com/giodl73-repo/TARMAC), [HARBOR](https://github.com/giodl73-repo/HARBOR) |
| Lifelines | [PYLON](https://github.com/giodl73-repo/PYLON), [PACKET](https://github.com/giodl73-repo/PACKET), [BASIN](https://github.com/giodl73-repo/BASIN), [DRAIN](https://github.com/giodl73-repo/DRAIN) |
| Public access | [SHIELD](https://github.com/giodl73-repo/SHIELD), [SLATE](https://github.com/giodl73-repo/SLATE) |
| Civic boundaries | [ZONES](https://github.com/giodl73-repo/ZONES) |

The family shares evidence labels, explicit scale and demand bases, T1–T4
service promises where meaningful, adversarial review, and acceptance of a
rigorous null result. Each repository owns its domain semantics and safety
boundary.

> SHIELD is not a clinical study, licensing/accreditation determination,
> Certificate-of-Need or payer decision, medical advice, or advocacy brief, and
> it claims no CMS, state-agency, health-system, payer, or provider endorsement.

## Use SHIELD

SHIELD is public and open to use as a reference model for aggregate,
evidence-gated healthcare-access analysis. To scope a safe transfer test,
source review, or aggregate-only local adaptation, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

## Why this matters

Healthcare capacity is non-fungible: the wrong bed, clinician, specialty,
coverage, referral path, or travel time cannot be repaired by a single total.
SHIELD deliberately tests where physical-infrastructure scoring transfers—and
where it breaks—on a service-and-human network.

## First public-data result: the hospital footprint

SHIELD now reproduces the May 13, 2026 CMS Hospital General Information
release as a bounded national denominator:

| What CMS reports | Facilities |
|---|---:|
| Medicare-registered hospitals | 5,432 |
| Flagged `Emergency Services = Yes` | 4,498 |
| Critical Access Hospitals | 1,378 |
| Rural Emergency Hospitals | 41 |

The source contains 5,432 unique facility IDs across 56 states and included
territories. Its hospital-type, emergency-service, and geography partitions
each reconcile independently to the same denominator.

This is useful because it establishes *where to start asking access questions*.
It does not answer them. Facility presence is not travel access; an emergency
flag is not a staffing or response-time SLA; and unlike hospital types cannot
be added as interchangeable capacity. Travel time, staffed beds, clinicians,
service breadth, wait time, affordability, quality, outcomes, equity, need,
costs, and savings all remain held.

### County rurality distribution

The second public-data slice joins that footprint to USDA ERS 2023
Rural-Urban Continuum Codes without fuzzy matching or hand-written aliases:

| Deterministic join result | Facilities |
|---|---:|
| Matched to a RUCC county/county-equivalent | 5,360 |
| Metro county (RUCC 1–3) | 3,456 |
| Nonmetro county (RUCC 4–9) | 1,904 |
| Unmatched and left unallocated | 72 |

Among matched facilities, 1,086 of 1,371 Critical Access Hospitals and 36 of
41 Rural Emergency Hospitals are in nonmetro counties. This describes county
class, not distance, travel time, patient rurality, staffing, shortage, need,
service quality, or adequacy. The 72 unmatched rows remain visible rather than
being silently assigned.

### Primary-care shortage registry

SHIELD also reproduces the July 31, 2026 HRSA primary-care HPSA download at
its actual registry grain:

| Current HRSA CSV result | Count |
|---|---:|
| Designation-component rows | 79,150 |
| Unique currently `Designated` HPSA IDs | 7,682 |
| Unique `Proposed For Withdrawal` HPSA IDs | 1,014 |
| Unique `Withdrawn` HPSA IDs | 8,999 |
| Currently designated IDs spanning multiple components | 762 |
| Currently designated IDs spanning multiple rural-status values | 282 |

HRSA's separate June 30 quarterly report counts 9,003 primary-care HPSA
designations. SHIELD preserves that official total but does not force it to
match the newer daily file: the sources have different dates and status
surfaces. Component rows are not hospitals, counties, or people, and
designation populations can overlap. This result therefore establishes a
formal shortage-registry spine without assigning shortage, access, capacity,
adequacy, costs, or savings to any hospital or community.

### Designation–component–geography bridge

The same July 31 file now yields a geography bridge without flattening
subcounty or facility designations into whole-county findings:

| Current designation structure | HPSA IDs | Component rows | Multi-component IDs | Multi-county IDs |
|---|---:|---:|---:|---:|
| Single County components | 2,088 | 2,248 | 89 | 89 |
| Census Tract components | 586 | 11,697 | 555 | 25 |
| County Subdivision components | 164 | 1,579 | 118 | 41 |
| **All area designations** | **2,838** | **15,524** | **762** | **155** |
| Facility designations | 4,844 | 4,844 | 0 | 0 |

All 15,524 area-component rows have internally consistent five-digit common
county keys. Among facility designations, 4,826 do and 18 remain an explicit
geography residual: 17 placeholder keys and one state-prefix inconsistency.
Across both classes, 7,664 of 7,682 designation IDs have a validated common
county key spanning 2,932 distinct codes.

This bridge locates designation components; it does not assign a designation
to a CMS hospital, turn a tract or subdivision finding into whole-county
shortage, deduplicate affected people, or establish access, staffed capacity,
need, or adequacy.

### Primary-care designation capacity formulas

The same current registry identifies exactly where HRSA records primary-care
physician FTE and shortage values:

| Formula coverage | HPSA IDs | Recorded FTE | Recorded shortage | Derived need met |
|---|---:|---:|---:|---:|
| Area designations | 2,838 | 10,327.3034 | 11,498.4866 | 47.32% |
| Correctional facilities | 550 | 308.1850 | 768.6050 | 28.62% |
| **Capacity-bearing designations** | **3,388** | **10,635.4884** | **12,267.0916** | **46.44%** |

All 2,838 area records reproduce the served-population formula within half a
person and the shortage formula within 0.01 FTE. Their aggregate population
identity has a visible one-person rounding residual. The remaining 4,294
facility designations have no FTE or shortage value: 1,351 FQHCs, 171 FQHC
Look-Alikes, 928 IHS/Tribal/Urban Indian organizations, nine Other Facilities,
and 1,835 Rural Health Clinics.

These are designation-recorded primary-care physician quantities, not
deduplicated people or clinicians. HRSA's formula excludes nurse-practitioner
and physician-assistant services and specified automatic or service-based
facility designations. The result is therefore a valid formula baseline, not a
CMS hospital staffing, appointment-access, service-line, or adequacy result.

### Hospital operational-capacity spine

CMS's 2023 Hospital Provider Cost Report adds a historical operational layer
at report-record grain and joins to the current hospital footprint by exact
CMS Certification Number (CCN):

| Operational result | Count or rate |
|---|---:|
| Cost-report records / unique CCNs | 6,103 / 6,040 |
| Current hospital IDs with an exact cost-report CCN | 5,144 of 5,432 (94.70%) |
| Current hospital IDs with at least one usable operational report | 5,032 (92.64%) |
| Usable reports / missing / invalid | 5,953 / 125 / 25 |
| Valid available bed-days / inpatient days | 241,546,243 / 151,101,088 |
| Bed-day-weighted inpatient use | 62.56% |
| Current-footprint matched weighted use | 62.33% |

The 62 repeated CCNs represent 63 adjacent, non-overlapping reporting-period
pairs, so valid report-period days can be combined without double-counting
time. Point-in-time bed values are not added across those reports. The 25
invalid records have inpatient days above available bed-days and remain an
explicit residual; 125 records lack at least one required operational field.

CMS defines the bed measure as adult and pediatric beds available for patient
use, not staffed beds. Accordingly, weighted use is an operational observation,
not proof of staffed capacity, service-line availability, surge readiness,
patient access, local need, quality, or adequacy. It supplies a shared CCN
identity and utilization spine for the next source, not a funding candidate.

### Certified services and recorded workforce

CMS's Q2 2026 QIES Provider of Services file supplies that next identity layer.
An exact CCN join covers 5,422 of 5,432 current hospitals (99.82%). Fourteen
service fields are complete for 5,286 current hospitals (97.31%); the 136-row
service residual is entirely 112 Veterans Administration and 24 Department of
Defense hospitals.

| Example certified service | Staff only | Arrangement only | Both | Not provided | Missing |
|---|---:|---:|---:|---:|---:|
| Dedicated emergency department | 2,494 | 209 | 1,651 | 932 | 136 |
| Inpatient surgery | 3,215 | 74 | 664 | 1,333 | 136 |
| Medical/surgical ICU | 2,670 | 59 | 414 | 2,143 | 136 |
| Obstetrics | 2,382 | 85 | 379 | 2,440 | 136 |

The same matched rows contain complete fields for seven employed-workforce
categories, including registered nurses, physicians, nurse practitioners, and
physician assistants. SHIELD preserves source-recorded zeros and conspicuous
maximum values instead of silently cleaning or interpreting them. These values
are recorded provider FTEs—not unique people, shifts, hours, vacancies, agency
staff, appointment supply, throughput, or current service coverage.

Certification establishes a reported delivery mode (`staff`, `under
arrangement`, both, or not provided); it does not prove that a service is open
now or has enough staff. The result is therefore a service/workforce evidence
spine for a future access test, not a capacity, adequacy, or savings claim.

### Emergency-care timeliness observations

The May 13, 2026 CMS Timely and Effective Care release adds historical
emergency-process observations by exact facility ID:

| Emergency-care result | Facilities or value |
|---|---:|
| Current hospitals with a standard reporting row | 4,660 of 5,432 (85.79%) |
| Overall ED-time values / unavailable | 4,050 / 610 |
| Facility median / CMS national value | 154 / 167 minutes |
| Transfer-time values / unavailable | 2,340 / 2,320 |
| Left-without-being-seen values / unavailable | 3,832 / 828 |
| Separate Rural Emergency Hospital reporters | 41 |

The 772 current hospitals outside the standard reporting surface are 635
psychiatric, 132 Veterans Administration, and five long-term hospitals. All 41
Rural Emergency Hospitals have a distinct CMS reporting file; SHIELD does not
mistake their suppressed standard rows for missing facilities.

CMS values cover stated 2024 or July 2024–June 2025 periods. Facility medians
are not live waits, cannot be averaged into a patient-weighted national result,
and do not reveal why a facility differs from the national value. That value is
a descriptive reference—not an access or adequacy floor. Current schedules,
staffing, travel/catchments, need, causal effects, costs, and savings remain
held.

### County emergency-use demand context

CMS's 2024 Medicare Geographic Variation file supplies a resident-demand
denominator for Original Medicare:

| County-demand result | Count or value |
|---|---:|
| County rows / usable ED-use rates | 3,197 / 3,143 |
| Suppressed or missing ED-use rates | 54 |
| National ED visits / Original Medicare beneficiaries | 16,377,193 / 27,732,177 |
| National / county-median visits per 1,000 | 590.5484 / 606.6914 |
| Current facilities placed in covered counties | 5,300 |
| Demand counties with / without a current hospital | 2,435 / 762 |
| No-hospital counties with numeric demand | 708 |

Those 708 counties contain 1,527,795 Original Medicare beneficiaries and
906,563 recorded ED visits. This is a useful cross-county planning queue, but
not a finding that residents lack access: the source records beneficiary
residence, not the treating hospital, and does not observe cross-county travel.
It excludes Medicare Advantage and non-Medicare populations. Higher utilization
can reflect morbidity, substitution, availability, or practice patterns—not
necessarily unmet need. Travel time, catchments, total-population demand,
adequacy, candidates, costs, and savings remain held.

### Observed inpatient origin-destination flow

CMS's 2024 Hospital Service Area file supplies observed Medicare inpatient
hospital/beneficiary-mailing-ZIP pairs. SHIELD joins them by exact CCN to the
same-year Q4 Provider of Services hospital surface:

| Inpatient-flow result | Count or value |
|---|---:|
| HSA rows / providers | 1,156,702 / 7,536 |
| Numeric / suppressed pairs | 146,996 / 1,009,706 |
| Exact Q4 POS provider matches | 5,902 |
| Matched observable cases | 13,330,744 |
| Classified valid-origin cases | 13,330,468 |
| Different-ZIP cases | 11,586,529 (86.92%) |

The result confirms that inpatient care commonly crosses ZIP boundaries, so a
hospital inside or outside a resident's county cannot by itself describe the
care pathway. It does not show county crossings, road distance, travel time,
emergency-department destinations, reasons for travel, unique patients, burden,
or access failure. Suppressed pairs remain suppressed. Emergency-specific flow,
total-population demand, adequacy, candidates, costs, and savings remain held.

### National EMS destination context

The NEMSIS 2024 annual public data report adds a national 911/EMS routing
surface:

| EMS result | Count or value |
|---|---:|
| Total / 911 activations | 60,298,684 / 46,733,668 |
| Reporting agencies / states and territories | 14,756 / 54 |
| Destination-coded 911 events | 30,123,274 |
| Hospital / freestanding ED destinations | 27,706,728 / 156,346 |
| Combined ED destination share | 92.50% |
| Rural / frontier incident-urbanicity events | 2,652,293 / 452,100 |

This establishes that emergency departments dominate recorded 911 EMS
destinations. It does not establish county origin-destination flow or travel
time. NEMSIS submissions are voluntary, activations are not unique patients,
table denominators differ, remote-county coverage is not complete, and public
state/county/ZIP identifiers are restricted. Local access, need, adequacy,
candidates, costs, and savings remain held.

### Condition-specific modeled drive-time coverage

Minnesota Department of Health's July 2026 stroke-system map supplies the
first current, emergency-service-specific drive-time coverage benchmark:

| Minnesota stroke-system result | Count or value |
|---|---:|
| Designated stroke hospitals | 123 |
| Population within a modeled 30-minute drive | 97% |
| Population within a modeled 60-minute drive | 99% |

This is meaningful access evidence: it measures statewide population coverage
to facilities designated for a time-sensitive condition. It is not observed
ambulance travel, a patient origin-destination file, a county table, or a
national estimate. The published summary does not enumerate the uncovered
population or establish service readiness, unmet need, adequacy, candidates,
costs, or savings.

### Local EMS response-time context

FDNY's NYC OpenData dispatch file supplies a current local operational timing
surface for calendar 2025. SHIELD stores only exact aggregate query results,
not incident rows:

| NYC EMS result | Count or value |
|---|---:|
| Incidents / valid response-time events | 1,612,273 / 1,510,191 (93.67%) |
| Severity-1 valid events | 27,540 |
| Severity-1 mean response / travel time | 421.713 / 372.592 seconds |
| Severity-1 borough mean-response range | 388.215–456.609 seconds |

Response time runs from incident creation to the first unit on scene; travel
time runs from first-unit assignment to first unit on scene. The severity code
reflects information available to dispatch, not the patient's actual condition.
These are arithmetic means, and the source does not supply an accepted target,
percentiles, scene-to-hospital time, outcomes, population denominators, or a
causal explanation for borough differences. The result therefore does not
establish adequacy, inequity, a candidate intervention, cost, or savings.

### Response tails and official benchmark context

Exact aggregate response-second frequencies make the severity-1 distribution
visible without storing incident records:

| Calendar-2025 severity-1 result | Count or value |
|---|---:|
| Valid events | 27,540 |
| Median / p90 / p95 response | 366 / 650 / 792 seconds |
| At or below 10 minutes | 23,922 (86.86%) |
| Over 10 minutes | 3,618 |
| Borough p90 range | 593–698 seconds |

The Preliminary Fiscal 2026 Mayor's Management Report separately reports an
FY2025 ambulance response average of 8:49 for life-threatening emergencies and
an FY2026 target of 6:55. It also reports a combined ambulance/fire FY2025
average of 7:45 and FY2026 target of 6:00. These are useful official benchmark
definitions, but they cannot be scored against the extract above: fiscal year
does not equal calendar year, Segment 1–3 does not equal severity 1 alone, call
receipt does not equal incident creation, and arriving-unit scope is not proved
identical. The 10-minute extract share is also not Local Law Category 9 ALS
compliance. Adequacy, causes, outcomes, candidates, costs, and savings remain
held.

## Why this is harder than physical infrastructure

SHIELD cannot treat capacity as a fungible physical flow. A staffed bed,
specialist, clinic slot, payer pathway, referral route, transport option, and
continuity relationship are not interchangeable.

That makes the evidence boundary stricter:

- use aggregate and synthetic fixtures unless a source is explicitly public and
  safe;
- never introduce patient records or individual medical recommendations;
- keep licensing, payer, Certificate-of-Need, accreditation, and clinical claims
  held unless a qualified external authority and source path support them;
- treat transfer-strain findings as service-network evidence, not medical
  advice.

## What is implemented

| Crate | Responsibility |
|---|---|
| `shield-network` | Care-delivery elements and pathway contracts. |
| `shield-corpus` | Evidence-labelled corpus validation. |
| `shield-score` | DIM-01..13 score artifacts. |
| `shield-tier` | Tier-SLA classification and shortfalls. |
| `shield-gap` | Gap analysis, transfer-strain evidence, and null results. |
| `shield-cms-access` | Reconciled CMS/USDA facility, CMS operational-capacity, certified-service/workforce, emergency-process, inpatient-flow, NEMSIS destination, Minnesota stroke drive-time, NYC EMS response-time distribution/target and Local Law 119 Category 9 public-evidence boundary, HRSA shortage-registry, and held HLT baselines. |
| `shield-cli` | Corpus, score, tier-SLA, and gap commands. |

The implementation baseline is complete and fixture-backed. No patient records
or clinical recommendations belong in this repository.

## Quick start

```powershell
cargo run -p shield-cli -- --help
cargo run -p shield-cli -- cms-access-baseline
cargo run -p shield-cli -- cms-access-held-pack
cargo run -p shield-cli -- cms-rurality-baseline
cargo run -p shield-cli -- cms-rurality-held-pack
cargo run -p shield-cli -- hrsa-primary-care-baseline
cargo run -p shield-cli -- hrsa-primary-care-held-pack
cargo run -p shield-cli -- hrsa-geography-baseline
cargo run -p shield-cli -- hrsa-geography-held-pack
cargo run -p shield-cli -- hrsa-capacity-baseline
cargo run -p shield-cli -- hrsa-capacity-held-pack
cargo run -p shield-cli -- cms-operational-capacity-baseline
cargo run -p shield-cli -- cms-operational-capacity-held-pack
cargo run -p shield-cli -- cms-certified-services-workforce-baseline
cargo run -p shield-cli -- cms-certified-services-workforce-held-pack
cargo run -p shield-cli -- cms-emergency-care-timeliness-baseline
cargo run -p shield-cli -- cms-emergency-care-timeliness-held-pack
cargo run -p shield-cli -- cms-county-emergency-demand-baseline
cargo run -p shield-cli -- cms-county-emergency-demand-held-pack
cargo run -p shield-cli -- nyc-ems-response-time-baseline
cargo run -p shield-cli -- nyc-ems-response-time-held-pack
cargo run -p shield-cli -- nyc-ems-response-distribution-target-baseline
cargo run -p shield-cli -- nyc-ems-response-distribution-target-held-pack
cargo run -p shield-cli -- nyc-ems-local-law-119-category9-baseline
cargo run -p shield-cli -- nyc-ems-local-law-119-category9-held-pack
cargo run -p shield-cli -- nyc-ems-local-law-119-reporting-scope-baseline
cargo run -p shield-cli -- nyc-ems-local-law-119-reporting-scope-held-pack
cargo run -p shield-cli -- nyc-ems-category9-operations-context-baseline
cargo run -p shield-cli -- nyc-ems-category9-operations-context-held-pack
cargo run -p shield-cli -- nyc-ems-category9-public-evidence-boundary-baseline
cargo run -p shield-cli -- nyc-ems-category9-public-evidence-boundary-held-pack
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md)
- [`docs/adoption/`](docs/adoption)
- [`docs/vtrace/`](docs/vtrace)
- [`context/waves/`](context/waves)
- [`.roles/ROLE.md`](.roles/ROLE.md)

## License

MIT. See [`LICENSE`](LICENSE).

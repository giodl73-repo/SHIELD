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
| `shield-cms-access` | Reconciled CMS/USDA facility, CMS operational-capacity, CMS certified-service/workforce, HRSA shortage-registry, and held HLT baselines. |
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

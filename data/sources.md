# Public aggregate sources

## CMS-HGI-2026-05

- Publisher: Centers for Medicare & Medicaid Services (CMS)
- Dataset: Hospital General Information (`xubh-q36u`)
- Landing page: <https://data.cms.gov/provider-data/dataset/xubh-q36u>
- Released: 2026-05-13
- Modified: 2026-04-28
- Captured: 2026-07-31
- Source CSV rows / unique facility IDs: 5,432 / 5,432
- Source CSV bytes: 1,453,884
- Source CSV SHA-256: `83c98b2e8687580e0482b13e1e9acd5813534be243e5ccd9f55556a869595d40`
- Derived fixture: `data/derived/cms-hospital-footprint-2026-05-13.json`

SHIELD retains only aggregate counts by CMS hospital type, emergency-service
flag, and state or territory. It does not retain the facility rows in this
repository. Counts describe the CMS registry release; they do not establish
travel access, staffed capacity, service availability, quality, outcomes,
equity, need, adequacy, cost, savings, or causal effect.

## USDA-ERS-RUCC-2023

- Publisher: U.S. Department of Agriculture, Economic Research Service
- Dataset: 2023 Rural-Urban Continuum Codes
- Landing page: <https://www.ers.usda.gov/data-products/rural-urban-continuum-codes>
- Last updated: 2024-01-22
- Captured: 2026-07-31
- Source CSV rows: 9,703 long-form rows
- County/county-equivalent entities: 3,235; entities with RUCC values: 3,233
- Source CSV bytes: 629,322
- Source CSV SHA-256: `ec455ee2a8bc5fc8e070575ea5bee7dce46fc6037f8c3449cbf56e8b45331fa7`
- Derived fixture: `data/derived/cms-usda-rurality-join-2026-05-13.json`

The join uses state and an explicitly normalized county name. It applies no
fuzzy matching or manual aliases and reports 72 CMS facilities as unmatched.
RUCC is a county classification; it does not measure patient rurality, distance,
travel time, hospital service areas, staffing, shortage, need, or adequacy.

## HRSA-PC-HPSA-2026-07-31

- Publisher: Health Resources and Services Administration (HRSA)
- Dataset: Primary Care Health Professional Shortage Area detail CSV
- Download: <https://data.hrsa.gov/DataDownload/DD_Files/BCD_HPSA_FCT_DET_PC.csv>
- Created and captured: 2026-07-31
- Source component rows: 79,150
- Source CSV bytes: 48,303,351
- Source CSV SHA-256: `8ed0007ef82194e44a1ee9086723e42f79e5effe96104cf3104648f2837d2673`
- Derived fixture: `data/derived/hrsa-primary-care-hpsa-census-2026-07-31.json`

The file contains designation-component rows. SHIELD counts unique HPSA IDs
within each status and does not treat rows as independent designations,
hospitals, whole counties, or people. Designation populations can overlap.

The same-vintage geography bridge validates `Common State County FIPS Code` as
exactly five numeric characters whose state prefix equals `Common State FIPS
Code`. It retains 20,350 valid current-designation rows and 18 facility-row
residuals (17 placeholders and one state-prefix inconsistency). It does not
repair those rows from alternate fields. The derived fixture is
`data/derived/hrsa-primary-care-geography-bridge-2026-07-31.json`.

## HRSA-HPSA-QUARTERLY-2026-Q2

- Publisher: Health Resources and Services Administration (HRSA)
- Report: Designated Health Professional Shortage Areas Statistics
- Report: <https://data.hrsa.gov/Default/GenerateHPSAQuarterlyReport?stream=top>
- Data as of: 2026-07-01; designations as of: 2026-06-30
- Captured: 2026-07-31
- Source PDF bytes: 937,344
- Source PDF SHA-256: `e71178b1c88a6b00a8a4a28780ecbff1f75e39f490ea43a8e5c5c899e211f89c`
- Primary-care HPSA designations: 9,003

The quarterly total is retained at its stated vintage. It is not reconciled to
the newer daily CSV because the two sources have different dates and status
surfaces.

## HRSA-HPSA-METADATA-2026-07-31

- Publisher: Health Resources and Services Administration (HRSA)
- Dataset: HPSA Data Mart Metadata
- Download: <https://data.hrsa.gov/DataDownload/DD_Files/HPSA_DATAMART_METADATA.XLSX>
- Captured: 2026-07-31
- Source XLSX bytes: 27,297
- Source XLSX SHA-256: `0439dbf209f0b05745e99cc0911e49262b422c564282a48b9030a6175b7ab91b`
- Derived fixture: `data/derived/hrsa-primary-care-designation-capacity-2026-07-31.json`

The capacity baseline deduplicates component rows by HPSA ID and retains FTE,
shortage, ratio-goal, and population-formula values only where HRSA supplies
them. The quarterly report's endnotes define the primary-care need-met and
practitioner-needed formulas, exclude specified facility classes, and state
that nurse-practitioner and physician-assistant services are not included. The
derived current-file aggregate is not reconciled to the different-vintage
quarterly total and is not a unique workforce or population count.

## CMS-HOSPITAL-COST-REPORT-2023

- Publisher: Centers for Medicare & Medicaid Services (CMS)
- Dataset: Hospital Provider Cost Report, version 2023
- Landing page: <https://data.cms.gov/provider-compliance/cost-reports/hospital-provider-cost-report>
- Dataset version ID: `cb8d0018-1bbe-4559-91bf-9429ac344b48`
- Released: 2026-01-08
- Captured: 2026-07-31
- Source rows / unique report IDs / unique CCNs: 6,103 / 6,103 / 6,040
- Source CSV bytes: 4,130,294
- Source CSV SHA-256: `614f3d94dfeb84092ca775f90913ab4f843233a4fa90c3df3013efeb5221a757`
- Data dictionary bytes / SHA-256: 377,560 / `40342bb506d4c39ad6a3306d188f414e97e683c8c7241aa5e0975bdc59c87b81`
- Methodology bytes / SHA-256: 49,641 / `e216d1f303095bdd57acec09c9f11b70c5188c8ee3460d201a5556db64d9b410`
- Derived fixture: `data/derived/cms-hospital-operational-capacity-2023.json`

The source contains annual hospital cost-report records, not one row per
current facility. Sixty-two CCNs repeat across 63 adjacent, non-overlapping
reporting-period pairs. SHIELD retains report grain, combines only valid
report-period bed-days and inpatient days, and does not add point-in-time bed
counts across reports. Of 6,103 records, 5,953 have nonnegative available beds
and inpatient days with positive bed-days and inpatient days no greater than
available bed-days; 125 lack a required field and 25 fail that relation.

Exact CCN identity matches 5,144 of the 5,432 hospitals in the May 13, 2026 CMS
footprint; 5,032 current IDs have at least one usable operational report. The
different vintages and both residuals remain explicit. CMS's available-bed
definition is not a staffed-bed definition, and weighted inpatient use does
not establish service-line capacity, access, need, surge readiness, quality,
adequacy, costs, or savings.

## CMS-QIES-POS-2026-Q2

- Publisher: Centers for Medicare & Medicaid Services (CMS)
- Dataset: Provider of Services File - Quality Improvement and Evaluation System
- Landing page: <https://data.cms.gov/provider-characteristics/hospitals-and-other-facilities/provider-of-services-file-quality-improvement-and-evaluation-system>
- Dataset type/version IDs: `8ba0f9b4-9493-4aa0-9f82-44ea9468d1b5` / `bb342fae-b551-40fd-a738-e2e5878f3bbb`
- Vintage / released / captured: Q2 2026 / 2026-07-16 / 2026-07-31
- Source CSV rows / columns: 44,707 / 473
- Hospital rows / unique CCNs: 13,566 / 13,566
- Source CSV bytes / SHA-256: 30,195,693 / `bcfb9c680f02fdc05a4c82b90e434ad96bb52ea0a1f04eb424bb3dad5a9ffe3d`
- Data dictionary bytes / SHA-256: 6,141,361 / `08291e4cd5d0221b1201d48be1d58a916af9ea4b608b90c4ebc5ae450ef1f4b7`
- Methodology bytes / SHA-256: 53,334 / `3f9c45bc360101b8ccb0d9107a912fd0f703e9ef7cf060028b35c29fee15705b`
- Derived fixture: `data/derived/cms-certified-services-workforce-2026-q2.json`

Exact CCN identity matches 5,422 of 5,432 hospitals in the current HGI
footprint. Service-mode fields are complete for 5,286; the 136 missing rows are
112 Veterans Administration and 24 Department of Defense hospitals. CMS codes
services as not provided, provided by staff, provided under arrangement, or
both. Those codes do not establish present schedules, hours, throughput, or
adequacy. Seven workforce fields are retained as provider-recorded employed
FTE values. They are not unique people or current shift coverage, and source
zeros and conspicuous maxima remain visible pending compatible validation.

## CMS-TEC-2026-05

- Publisher: Centers for Medicare & Medicaid Services (CMS)
- Dataset: Timely and Effective Care - Hospital (`yv7e-xc69`)
- Landing page: <https://data.cms.gov/provider-data/dataset/yv7e-xc69>
- Released / modified / captured: 2026-05-13 / 2026-04-28 / 2026-07-31
- Provider CSV rows / bytes / SHA-256: 138,173 / 34,178,467 / `5d39e1fd8b7b272fe83f7b53e2f69288c997dfb4d28b68dd74454e80e7d860e9`
- National CSV rows / bytes / SHA-256: 45 / 13,805 / `e71b0a16dc71eb9826b1d7cc4eab5c3bddbc1a285681f28e2ae33a9c2b8628e9`
- Rural Emergency Hospital provider CSV rows / bytes / SHA-256: 164 / 59,753 / `c83bdee86d813a9a23b642cc3ed159825cef355e9f025c274f50f64cd12568e0`
- Rural Emergency Hospital national CSV rows / bytes / SHA-256: 8 / 2,822 / `ed2328cd063920a0cd45c5ee36d13f91983657a161affaf73c0d76115203d1af`
- Data dictionary bytes / SHA-256: 1,291,356 / `cd5016abee26e914b273a8fea8ab698710ff60f1c53a1b66e43bbd7168f6cb81`
- Derived fixture: `data/derived/cms-emergency-care-timeliness-2026-05.json`

The standard provider file contains seven emergency rows for each of 4,660
facilities and matches all of them to the current HGI footprint by exact CCN.
The 772 current IDs without a standard row remain classified by hospital type.
All 41 Rural Emergency Hospitals appear in a separate CMS reporting surface;
their standard rows carry footnote 19 and are not treated as usable measures.

Scores cover calendar 2024 or July 2024 through June 2025. SHIELD preserves
`Not Available` values and uses CMS national values only as descriptive
references. Facility medians are not live waits, patient-weighted system
estimates, access floors, need measures, causal effects, or adequacy findings.

## CMS-GEOGRAPHIC-VARIATION-2024

- Publisher: Centers for Medicare & Medicaid Services (CMS)
- Dataset: Medicare Geographic Variation - by National, State & County
- Landing page: <https://data.cms.gov/summary-statistics-on-use-and-payments/medicare-geographic-comparisons/medicare-geographic-variation-by-national-state-county>
- Dataset ID: `6219697b-8f6c-4164-bed4-cd9317c58ebc`
- Latest year / modified / captured: 2024 / 2026-05-15 / 2026-07-31
- Source CSV rows / columns: 36,994 / 246
- Source CSV bytes / SHA-256: 57,865,948 / `10c8304012da34da3ecfe4caf4548927095f693383814d0e79ce6711b6806fad`
- Data dictionary bytes / SHA-256: 563,924 / `75a8d4bef07d1900a50732c78a2aec688ba3ca132dad1dc6cab1a9243d55109f`
- Methodology bytes / SHA-256: 196,478 / `e7c6ca8a3cb4cd761f44ee5d5e4ee78a379479a73cd93f69fb116c860a9944ca`
- Derived fixture: `data/derived/cms-county-emergency-demand-2024.json`

The 2024 all-age county surface contains 3,197 unique county codes. SHIELD
retains 3,143 numeric ED-visit rates and 54 suppressed or missing values. Valid
county values sum to 27,595,186 Original Medicare beneficiaries and 16,322,783
ED visits; the separately reported national totals are 27,732,177 and
16,377,193, leaving visible residuals rather than imputing suppressed counties.

Exact QIES POS county FIPS places 5,300 current hospitals in 2,435 covered
counties. Another 762 covered counties contain no current hospital location;
708 of those have numeric ED demand. County is beneficiary residence, not
treating-facility location. This is Original Medicare utilization—not total
population, observed patient flow, travel access, unmet need, or adequacy.

## CMS-HSA-INPATIENT-FLOW-2024

- Publisher: Centers for Medicare & Medicaid Services (CMS)
- Dataset: Hospital Service Area, 2024
- Landing page: <https://data.cms.gov/provider-summary-by-type-of-service/medicare-inpatient-hospitals/hospital-service-area>
- Dataset version ID: `22e11819-48e8-44a8-a036-fd6351f4173a`
- Modified / captured: 2025-09-10 / 2026-07-31
- Source CSV rows / columns: 1,156,702 / 5
- Source CSV bytes / SHA-256: 26,951,355 / `1698b7fb8372ebafe126099e2b2406a841411f6eb2d22f50ca9add545bff5aa8`
- Data dictionary bytes / SHA-256: 70,220 / `06535c24d23404b14befa8248800dabd0503232f3166f5a5292e4c05384943d1`
- Methodology bytes / SHA-256: 69,971 / `ddd800c5d4904a30ae2885a5a0c98149edb4badd1a3ec919e007928b521390e6`
- Q4 2024 QIES POS rows / columns: 115,647 / 473
- Q4 POS bytes / SHA-256: 83,978,787 / `25cbd95b347dfc92130d4a4a3bc7c79dc7bbd8fbd1a16fc294029f22e7bfcb09`
- Derived fixture: `data/derived/cms-inpatient-origin-destination-2024.json`

The HSA source contains 146,996 numeric and 1,009,706 suppressed
hospital/origin-ZIP pairs. Exact CCN identity joins 5,902 HSA providers to the
same-year Q4 POS hospital surface. Of 13,330,468 matched observable cases with
a valid origin ZIP, 11,586,529 (86.92%) have an origin ZIP different from the
hospital ZIP.

The result observes inpatient discharge flow, not unique patients. Different
ZIP does not establish county crossing, distance, travel time,
emergency-department destination, reason for travel, burden, access failure,
unmet need, or adequacy.

## NEMSIS-EMS-DESTINATION-2024

- Publisher: National Emergency Medical Services Information System Technical Assistance Center
- Dataset: NEMSIS Data Report 2024
- Landing page: <https://nemsis.org/using-ems-data/articles-and-publications/>
- Report: <https://nemsis.org/wp-content/uploads/2025/09/NEMSIS-End-of-Year-Report-2024-9-24-25.pdf>
- Supported by: National Highway Traffic Safety Administration Office of Emergency Medical Services
- Created / captured: 2025-09-24 / 2026-07-31
- PDF pages / bytes / SHA-256: 18 / 2,054,076 / `64acb775e9b16f49427371e8b71f0dcda5dba5ec84bb92e004e1149e1f949a84`
- Derived fixture: `data/derived/nemsis-ems-destination-2024.json`

The report records 60,298,684 EMS activations from 14,756 agencies across 54
states and territories, including 46,733,668 911 activations. Its separately
grained destination table contains 30,123,274 coded events: 27,706,728 to
hospital emergency departments and 156,346 to freestanding emergency
departments. Together those are 92.50% of destination-coded events.

NEMSIS submissions are voluntary and activations are not unique patients. The
report says incidents occurred in 99% of U.S. counties but a small number of
remote counties did not contribute data. State/county/ZIP geographic
identifiers are restricted on the public surface. Destination type therefore
does not establish linked county flow, distance, travel time, local access,
unmet need, or adequacy.

## MN-MDH-STROKE-DRIVE-TIME-2026-07

- Publisher: Minnesota Department of Health
- Dataset: Minnesota Stroke System Coverage: Drive Time to Designated Stroke System Hospitals
- Landing page: <https://www.health.state.mn.us/diseases/cardiovascular/stroke/system.html>
- Map: <https://www.health.state.mn.us/diseases/cardiovascular/documents/drvtimemap.pdf>
- Page updated / map vintage / captured: 2026-07-21 / 2026-07 / 2026-07-31
- PDF pages / bytes / SHA-256: 1 / 333,247 / `490fc7ffc2c5af244ceafb35459b9b1c59907a6b66991a972d309d9ea1c130c4`
- Derived fixture: `data/derived/minnesota-stroke-drive-time-2026-07.json`

The official page reports 123 designated stroke hospitals. The July 2026 map
reports that 97% of Minnesotans live within a 30-minute drive and 99% within a
60-minute drive of a designated stroke-system hospital. These are published
statewide modeled population-coverage shares. They are not observed EMS trips,
patient origin-destination records, county estimates, a machine-readable
geographic surface, or national access estimates. The source does not publish
an exact uncovered population count in this summary.

## NYC-FDNY-EMS-DISPATCH-2025

- Publisher: Fire Department of New York City
- Dataset: EMS Incident Dispatch Data (`76xm-jjuj`)
- Landing page: <https://data.cityofnewyork.us/d/76xm-jjuj>
- API endpoint: <https://data.cityofnewyork.us/resource/76xm-jjuj.json>
- Rows updated / captured: 2026-07-15 / 2026-07-31
- Dataset rows at capture / incident range: 29,978,154 / 2005-01-01 through 2026-06-30
- Data dictionary bytes / SHA-256: 22,388 / `ccf797381643e39ebcd652892c730ab180bd751bedd9519b4cd9afc0e374a9de`
- Exact query-response SHA-256 values: snapshot `319e7f94b6a7efdb88a600d143f426915f332cc95497a266d821b29a7414eadf`; validity `a0338bc1c0a5ff4634fc15fa5e40a67a482110ace057350ef01bdc305df06aa7`; borough `1ba1c7912603c3f87a535545006b87851a10cec046e335e46018a22a2c2611dd`; severity `11f6674636f24ed419945624feb1023b607b3bc351fe1fda665d02ca8405b361`; severity-1 borough `e560a8ef184d5825b621a80baad8f14ff0e077835d88905a862ad3bf34e37296`
- Derived fixture: `data/derived/nyc-ems-response-time-2025.json`

Calendar 2025 contains 1,612,273 incidents; 1,510,191 have a valid response
time (93.67%). The 27,540 severity-1 valid events have a 421.713-second mean
response time and 372.592-second mean travel time. Response runs from incident
creation to first unit on scene; travel runs from first-unit assignment to
first unit on scene. Call type and dispatch severity do not establish the
patient's actual condition. Specific locations are removed, and SHIELD stores
no incident rows. Arithmetic means do not establish tails or targets; the
source does not establish scene-to-hospital time, outcomes, population rates,
causes, adequacy, candidates, costs, or savings.

## NYC-FDNY-EMS-DISTRIBUTION-TARGET-2025

- Dispatch publisher/dataset: FDNY / NYC OpenData EMS Incident Dispatch Data
- Dispatch landing page: <https://data.cityofnewyork.us/d/76xm-jjuj>
- Severity-1 citywide frequency response rows / bytes / SHA-256: 1,491 / 80,071 / `deb05f69e2d5a8bd239170d277527332974226f9a27a1ec535054730822d7445`
- Severity-1 borough frequency response rows / bytes / SHA-256: 4,504 / 340,061 / `59d87fdcea0beeb7347b265110ef76d3ad8cf62ea5c9ae008616acc567b7be41`
- Benchmark publisher: New York City Mayor's Office of Operations
- Report: Preliminary Fiscal 2026 Mayor's Management Report
- Report URL: <https://www.nyc.gov/assets/operations/downloads/pdf/pmmr2026/2026_pmmr.pdf>
- PDF bytes / SHA-256: 12,213,302 / `cdc002703628fbb71ae8b934c9a27654e13ace444b8300669e22792bd2b55f2f`
- Definitions: <https://www.nyc.gov/site/911reporting/reports/definitions.page>
- Derived fixture: `data/derived/nyc-ems-response-distribution-target-2025.json`

Nearest-rank reconstruction from exact integer-second frequency counts gives a
366-second median, 650-second p90, and 792-second p95 across 27,540 valid
calendar-2025 severity-1 records. Of those, 23,922 (86.86%) are at or below 600
seconds and 3,618 are above. Borough p90 values range from 593 to 698 seconds.

The official PMMR separately reports an FY2025 life-threatening-ambulance
average of 8:49 and FY2026 target of 6:55, plus a combined ambulance/fire
FY2025 average of 7:45 and target of 6:00. Direct comparison is blocked: the
surfaces differ on calendar versus fiscal year, severity 1 versus Segment 1–3,
incident creation versus call receipt, and unproved arriving-unit scope. The
derived at-or-below-ten-minute share is not Local Law Category 9, which is an
ALS-unit measure for qualifying ALS emergencies.

## NYC-LL119-CATEGORY9-2025

- Publisher: City of New York
- Dashboard: <https://www.nyc.gov/site/911reporting/reports/local-law-119-compliance.page>
- Definitions: <https://www.nyc.gov/site/911reporting/reports/definitions.page>
- Current code §15-129: <https://codelibrary.amlegal.com/codes/newyorkcity/latest/NYCadmin/0-0-0-25711>
- Power BI resource / model: `e59d7043-7f5c-4a62-bdec-295ddb7cc7b3` / `533915`
- Data as of / latest month / captured: 2026-07-27 / 2026-06 / 2026-07-31
- Conceptual schema bytes / SHA-256: 6,408 / `afae4549ef2811c72ea79f39adc12a3c4bf3b3bd22337096b7153869d2106bd7`
- Citywide query response bytes / SHA-256: 3,157 / `f238bfae46fcf541002a4e76e7522c8e8cfc94d6cd3476ef1aa0166dda53e4b9`
- Borough query response bytes / SHA-256: 4,550 / `81f6cfb984cd4cfaa14fdf3579962d0fa8e943ce1c2cfb4c1b2c8fcf0ca9c105`
- Derived fixture: `data/derived/nyc-ems-local-law-119-category9-2025.json`

The official calendar-2025 Category 9 measure is 39.26% across 216,599
qualifying incidents. Borough counts reconcile to the citywide count; named
borough shares range from 25.65% to 48.29%. The public model does not expose a
formula supporting an exact under-ten-minute numerator. Section 15-129 requires
reporting but specifies no minimum acceptable share; whole-law reporting
completeness and a legal or service pass/fail are therefore not claimed.

## NYC-LL119-REPORTING-SCOPE-2026

- Enacted law PDF: <https://legistar.council.nyc.gov/View.ashx?GUID=346763F6-8F97-4948-A56C-D87F8EF8FFF9&ID=6501297&M=F>
- PDF bytes / SHA-256: 170,028 / `6d703d17569b2329f17b0a44130a3907ac507af186001894dbfc15f00dd343c6`
- Models/exploration bytes / SHA-256: 188,914 / `530d4991f946f71500f141fca2c7a217519b8e66b5545dcfc92a24ad784856b5`
- Distinct-month query payload bytes / SHA-256: 612 / `72e94477a02809e18841f2003b85a0f00c97e2b64ac14ae346797d76d6d1a074`
- Distinct-month response bytes / SHA-256: 4,394 / `4a1a845d12181eb89868cc5e07a56b9dec30839bc59be10d7177f8e058fe7420`
- Derived fixture: `data/derived/nyc-ems-local-law-119-reporting-scope-2026.json`

The enacted law requires monthly and yearly reports to the Council and Mayor,
website posting, citywide results for each category, and borough
disaggregation. It does not require community-district or division reporting.
The public model contains 147 monthly labels from April 2014 through June 2026,
and the official page exposes citywide and borough views. Formal submission to
the Council and Mayor and a discrete yearly report were not verified, so full
statutory compliance remains unassessed.

## NYC-EMS-CATEGORY9-OPERATIONS-CONTEXT-2025

- Category 9 Power BI borough-month query rows / payload SHA-256 / response SHA-256: 72 / `83937ef408b57f8afd7f8a3cfdc0d5ae661b79951c85bc1097a067e0c5357ff3` / `4fbfea856a754207fdbe74c69cead3f3de9e57bc360e12d0dfaf06d94316036e`
- FDNY dispatch borough-month query rows / bytes / SHA-256: 66 / 19,992 / `144e10a419177ddd8a4eb8c23ddcbd4fe5f1ffe395175249773b27b251be63b3`
- FDNY metadata bytes / SHA-256: 50,138 / `85b190184d331ef66c31592bccf8b1d1ae811f061338c3b6a2ce8844e1a5aefe`
- Derived fixture: `data/derived/nyc-ems-category9-operations-context-2025.json`

Five named boroughs join across all 12 months, producing 60 ecological rows.
The screen covers 216,463 named-borough Category 9 qualifying incidents and
1,612,266 separately defined dispatch incidents. Reproducible Pearson moments
show descriptive co-movement, strongest for average dispatch travel seconds
(`r=-0.842`), average response seconds (`r=-0.759`), held share (`r=-0.740`),
and average dispatch seconds (`r=-0.702`). The qualifying sets and incident
identities do not match, borough/month confounding is uncontrolled, and neither
source contains a compatible patient-outcome join. These are hypothesis signals,
not operational drivers, causal effects, service adequacy, or candidates.

## NYC-EMS-CATEGORY9-PUBLIC-EVIDENCE-BOUNDARY-2025

- Direct Local Law 119 Open Data dataset: `gpny-cuvw`
- Dataset rows / first / last month: 11,323 / `2013 / 11` / `2026 / 06`
- Metadata bytes / SHA-256: 13,005 / `90894a60e7158d184e17e3f8e22b9e76457d0293a59aac1cc3a2afce66821d1e`
- Calendar-2025 Category 9 rows / bytes / SHA-256: 72 / 18,074 / `96c30e3ccdc493a7d0f887cb12858fb0c64fd1330ec48a430f533b6aad7a58c9`
- MMR dataset / metadata bytes / SHA-256: `rbed-zzin` / 54,846 / `e3d37f2030e7c168b8576938d551104d48256547a88a4edd33664f117bf6541a`
- MMR 2025 selected rows / bytes / SHA-256: 120 / 7,751 / `d74cdc3538de2460a0ec782f0f02da228f451631f905f195176e14f9211a28f3`
- 911 end-to-end dataset / metadata SHA-256: `t7p9-n9dy` / `b510a42b1e00fc3b773e36d7ce89478d38d8d47fe19e7b64f6f6b48bcc90a9e8`
- NYSDOH SPARCS ED summary / inpatient disposition datasets: `5gzv-zv2z` / `sf4k-39ay`
- NYC catalog posting/routing and cardiac-outcome zero-result response SHA-256: `f69ff5b2af3640908961a4a591987a87f66754c842a0172947ed372e935ea33b` / `90ca77160bd61336e8942e0683517998479b3e77b6c3a4b5f9fcdc75f773f31e`
- Derived fixture: `data/derived/nyc-ems-category9-public-evidence-boundary-2025.json`

The direct Open Data rows total 216,986 incidents, 387 more than the captured
Power BI model. Their weighted share, reconstructed from rounded monthly rows,
is 39.232% versus 39.260% in Power BI. The source-revision reason is unknown, so
the two sources are not blended. Open Data also misdescribes its monthly label
as a week-start date; that metadata defect remains visible.

Seven public source classes were tested. MMR borough indicator shells contain
no 2025 values; the other operations sources lack a matching ALS qualifying set
or shared incident key. SPARCS has privacy-safe disposition fields but no EMS
linkage. Two-way borough/month demeaning reduces average-response and dispatch
associations to `r=-0.187` and `r=-0.091`; travel remains the largest descriptive
signal at `r=-0.481`. This prioritizes acquisition of incident-linked ALS
availability/posting exposure and patient outcomes. It does not identify a
driver or support an intervention, adequacy, cost, savings, allocation, or rate
claim.

## NYC-EMS-CATEGORY9-SOURCE-DRIFT-2025

- Open Data dataset / updated: `gpny-cuvw` / `2026-07-22T18:35:35Z`
- Open Data metadata bytes / SHA-256: 1,901 / `55682468b7f7e02877b527d28fd2815abe7238020c25fd90be2e1a0b7b686273`
- Open Data data-dictionary bytes / SHA-256: 68,908 / `ace6fdaf9b75f51737e8df0833f15460d2cedfdc6372df57ca86ba9312ce6a6b`
- Open Data 72-cell response bytes / SHA-256: 18,074 / `96c30e3ccdc493a7d0f887cb12858fb0c64fd1330ec48a430f533b6aad7a58c9`
- Power BI resource / model / last refresh: `e59d7043-7f5c-4a62-bdec-295ddb7cc7b3` / `533915` / `2026-07-27T15:07:31.633Z`
- Power BI model/exploration bytes / SHA-256: 188,914 / `530d4991f946f71500f141fca2c7a217519b8e66b5545dcfc92a24ad784856b5`
- Power BI conceptual-schema bytes / SHA-256: 6,408 / `afae4549ef2811c72ea79f39adc12a3c4bf3b3bd22337096b7153869d2106bd7`
- Power BI 72-cell payload bytes / SHA-256: 2,454 / `c58c2099de8d47b8567c6c8bb43b102846d0a3be35aa59f78a12b59a315f4ead`
- Power BI response bytes / SHA-256: 5,197 / `b5dc59a54379d1fdbb0f9f2215f42728daf484278d5a1d5f7b87da349fc26495`
- Derived fixture: `data/derived/nyc-ems-category9-source-drift-2025.json`

Open Data totals 216,986 incidents and Power BI totals 216,599. In the exact
72-cell join, Open Data is higher in 54 cells, equal in 18, and lower in none;
monthly and borough deltas each sum to 387. Every month and borough label has a
nonzero aggregate difference, and the largest cell difference is 23 incidents.
The Power BI response hash records this capture but is not a deterministic
replay key because the response envelope contains runtime identifiers and
timestamps; the payload hash and derived cell fixture carry stable query and
arithmetic custody.

The City data dictionary says the Mayor's Office of Operations maintains the
monthly Citywide Performance Reporting data, but no revision mechanism or row
history was located. The pattern is consistent with broad snapshot revision;
it does not prove the cause or support an error claim. Keep both snapshots,
label their refresh times and precision, and rerun the full join when either
source refreshes.

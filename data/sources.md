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

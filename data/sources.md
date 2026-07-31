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

# WP-008: CMS–USDA County Rurality Join

Status: **accepted**

## Objective

Join SHIELD's CMS hospital denominator to USDA ERS 2023 Rural-Urban Continuum
Codes (RUCC) at county/county-equivalent grain and expose the matched metro and
nonmetro facility distribution. This is a geography-classification feature,
not a travel-access, shortage, capacity, or adequacy model.

## Source and join custody

| Source | Accepted custody |
|---|---|
| CMS Hospital General Information | Release 2026-05-13; 5,432 rows; SHA-256 `83c98b2e8687580e0482b13e1e9acd5813534be243e5ccd9f55556a869595d40` |
| USDA ERS 2023 RUCC | Updated 2024-01-22; 9,703 long-form rows; SHA-256 `ec455ee2a8bc5fc8e070575ea5bee7dce46fc6037f8c3449cbf56e8b45331fa7` |

The deterministic key is state plus a case-folded, diacritic-folded county
name after removing only declared county-equivalent suffixes. No fuzzy match or
manual alias is allowed. Unmatched rows stay visible.

## Product surfaces

- `data/derived/cms-usda-rurality-join-2026-05-13.json`: aggregate join fixture.
- `shield cms-rurality-baseline`: reconciled matched and unmatched distribution.
- `shield cms-rurality-held-pack`: fourteen-section HLT pack with no fiscal or
  healthcare-adequacy authority.

## Claim contract

Allowed: the count of CMS facilities that deterministically join to RUCC 2023,
their RUCC code and metro/nonmetro distribution, and the unmatched residual.

Held: patient rurality, distance, travel time, catchments, hospital service
areas, staffed beds, clinicians, service availability, shortage designation,
need, quality, outcomes, equity, adequacy, costs, savings, or candidate effects.

RUCC classifies counties, not patients or facility performance. A nonmetro
facility is not automatically accessible; a metro facility is not automatically
adequate. Unmatched rows cannot be allocated by assumption.

## Verification and role-review fixed point

- Matched plus unmatched facilities reconcile to 5,432.
- Metro plus nonmetro facilities reconcile to the matched denominator.
- RUCC codes and hospital-type partitions independently reconcile.
- Match, class-share, and selected within-class basis points recompute exactly.
- Health-system, clinical, operations, health-economics/payer, equity/public-
  health, citation, numeracy, and scope lenses all require the held boundaries
  above; no critical or major finding remains.

Fixed point: accepted for implementation. The 72 unmatched facilities remain a
reported source-interface residual, not an invitation to silent repair.

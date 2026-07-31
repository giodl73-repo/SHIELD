# WP-007: CMS Hospital Footprint Baseline

Status: **accepted**

## Objective

Turn one current, public CMS aggregate into a reproducible national facility-footprint result and a
fourteen-section held HLT handoff for Taxlane. This is a facility-presence feature, not a new project
management layer and not a healthcare-adequacy score.

## Source and custody

| Field | Accepted value |
|---|---|
| Publisher | Centers for Medicare & Medicaid Services (CMS) |
| Dataset | Hospital General Information (`xubh-q36u`) |
| Released | 2026-05-13 |
| Modified | 2026-04-28 |
| Capture date | 2026-07-31 |
| Source rows | 5,432 |
| CSV SHA-256 | `83c98b2e8687580e0482b13e1e9acd5813534be243e5ccd9f55556a869595d40` |

The repository keeps a derived aggregate fixture, not the source facility rows. The fixture must
reconcile hospital-type, emergency-service, and state/territory counts independently to 5,432.

## Product surfaces

- `crates/shield-cms-access`: feature-specific custody, reconciliation, baseline, and held-pack logic.
- `data/derived/cms-hospital-footprint-2026-05-13.json`: aggregate fixture with source metadata.
- `shield cms-access-baseline`: visible, deterministic national result.
- `shield cms-access-held-pack`: HLT handoff with no fiscal, allocation, rate, or public-release authority.

## Claim contract

Allowed: counts of Medicare-registered hospitals by CMS hospital type, emergency-service flag, and
state or territory in this release.

Held: travel time, geographic coverage, staffed beds, clinicians, service-line capacity, wait time,
affordability, quality, safety, outcomes, equity, need, adequacy, resilience, causal effects, costs,
savings, candidate selection, allocation, and tax-rate changes.

Facility presence is not access. An emergency-service flag is not a verified emergency-care SLA.
Hospital types are CMS source categories and are not interchangeable units of capacity.

## Verification and exit

- Fixture totals reconcile and facility IDs were unique in the captured source.
- Baseline exposes the source denominator and allowed/held interpretation.
- Held pack contains all fourteen Taxlane sections, uses track `HLT`, and keeps admission and every
  fiscal authority false or null.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and
  `cargo test --workspace --locked` pass.

## Role review fixed point

| Lens | Finding | Disposition |
|---|---|---|
| Health-system planner | Facility distribution is a denominator, not a referral-network or access result. | claim boundary required; accepted |
| Clinician / care delivery | Registration and emergency flag do not prove staffing, readiness, or safe care. | held explicitly; accepted |
| Operations & capacity | Facility counts cannot be converted to beds, slots, throughput, or surge capacity. | held explicitly; accepted |
| Health economist / payer realist | The source contains no comparable costs, utilization, prices, payer incidence, or savings. | fiscal fields null; accepted |
| Equity / public health | State counts cannot establish within-state reach, need, disparities, or outcomes. | no adequacy or equity claim; accepted |
| Citation auditor | Publisher, dataset ID, dates, URL, row count, capture date, and checksum are fixed. | pass |
| Numeracy checker | Type, emergency, and geography partitions must each reconcile to the source denominator. | executable invariant; pass |
| Scope keeper | A dedicated feature crate satisfies the shared-code rule without adding workflow infrastructure. | pass |

Fixed point: no critical or major finding remains. WP-007 is accepted for implementation with the
held-claim boundary above.

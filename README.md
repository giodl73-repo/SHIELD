# SHIELD

**Healthcare Access 2.0 — multi-scale care-delivery network analysis.**

**A bed is not access if the patient cannot reach, enter, afford, or continue through care.**

SHIELD scores facilities, providers, service lines, and referral or transfer
pathways across access, capacity, quality, workforce, affordability,
continuity, equity, and surge resilience.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> SHIELD is not a clinical study, licensing/accreditation determination,
> Certificate-of-Need or payer decision, medical advice, or advocacy brief, and
> it claims no CMS, state-agency, health-system, payer, or provider endorsement.

## Why this matters

Healthcare capacity is non-fungible: the wrong bed, clinician, specialty,
coverage, referral path, or travel time cannot be repaired by a single total.
SHIELD deliberately tests where physical-infrastructure scoring transfers—and
where it breaks—on a service-and-human network.

## What is implemented

| Crate | Responsibility |
|---|---|
| `shield-network` | Care-delivery elements and pathway contracts. |
| `shield-corpus` | Evidence-labelled corpus validation. |
| `shield-score` | DIM-01..13 score artifacts. |
| `shield-tier` | Tier-SLA classification and shortfalls. |
| `shield-gap` | Gap analysis, transfer-strain evidence, and null results. |
| `shield-cli` | Corpus, score, tier-SLA, and gap commands. |

The implementation baseline is complete and fixture-backed. No patient records
or clinical recommendations belong in this repository.

## Quick start

```powershell
cargo run -p shield-cli -- --help
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md)
- [`docs/vtrace/`](docs/vtrace)
- [`context/waves/`](context/waves)
- [`.roles/ROLE.md`](.roles/ROLE.md)

## License

MIT. See [`LICENSE`](LICENSE).

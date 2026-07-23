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
- [`docs/adoption/`](docs/adoption)
- [`docs/vtrace/`](docs/vtrace)
- [`context/waves/`](context/waves)
- [`.roles/ROLE.md`](.roles/ROLE.md)

## License

MIT. See [`LICENSE`](LICENSE).

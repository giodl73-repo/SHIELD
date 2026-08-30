# Pulse 07 - PITFALL Doctrine

## Scope

Add SHIELD's PITFALL doctrine index for recurring healthcare-access evidence
failure classes, then tie it back to README discovery, VTRACE evidence, public
source waves, and the role panel.

## Findings

- Facility footprints, emergency flags, county rurality, or HPSA registry
  presence must not become patient access, staffed capacity, service quality,
  adequacy, or availability claims.
- HRSA component rows, designation IDs, formula values, and facility
  designations must not become deduplicated people, clinicians, hospitals,
  service lines, or appointment access.
- Unmatched rows, invalid county keys, multi-component designations,
  multi-county designations, rounding residuals, and policy-excluded facility
  records must remain visible.
- Held HLT packs must not become clinical, facility, fiscal, rate, release, or
  Taxlane authority.
- The aggregate public-source baselines remain an open misuse risk until the
  next access claim records scale, source grain, demand basis, and full
  parliament/editorial review.

## Integration

- `.pitfall/PITFALL.md` indexes SHIELD principles, invariants, and pitfalls.
- `.pitfall/shield-principles.md` maps durable decision rules to README,
  CLAUDE, product, VTRACE, source, wave, role, and code evidence.
- `.pitfall/shield-invariants.md` records patient-record, source-grain,
  residual, formula, and held-pack properties.
- `.pitfall/shield-pitfalls.md` records mitigated and open failure classes for
  reuse in public-source and future access-claim reviews.

## Validation

Planned before commit:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p shield-cli -- --help
git diff --check
```

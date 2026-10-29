# Frozen primary-care geography source

This view replays the unchanged July 31, 2026 Designated HRSA geography
aggregate and native baseline/held-pack serializers (WP-010, WP-WEB-003).
It does not query the current registry. A reader can select all Designated
geography, area/facility designations or one recorded area component class.
No individual county counts, per-person records, identities or polygons are
provided; a map would invent spatial coverage absent from this aggregate.

Component rows, designation IDs and distinct common county keys are different
units. County-key counts overlap across cohorts/classes and must not be summed.
The 2,932-key union is context, not 2,932 whole counties in shortage. Tracts and
subdivisions remain partial-area components. Single County is a component class,
including 89 multi-county/multi-component IDs; its label does not make every
designation single-county. Area and facility partitions, component multiplicity
and county multiplicity are separate. Facility designations are not CMS hospitals.

The fixed Designated county-key check retains 18 residual rows / IDs: 17
placeholders and one state-prefix inconsistency, all facility records. No repair
or reassignment. Class-specific residuals in selected projections remain null.
Structural five-digit/state-prefix validity is not an independent county-master
verification. No county polygons, coverage area or deduplicated people follow.

Download geography-source.json: byte-for-byte copy of
data/derived/hrsa-primary-care-geography-bridge-2026-07-31.json. Its derivative
hash is recorded in source-custody.json. The original CSV is not in the portable
bundle and was not reacquired or reverified here. Its historical record is
48,303,351 bytes / SHA-256
8ed0007ef82194e44a1ee9086723e42f79e5effe96104cf3104648f2837d2673.
The publisher endpoint may now return a newer release; it is not a promised
frozen download. Existing source attribution and third-party terms apply.

Native replay: cargo run --locked -p shield-web --bin shield-browser -- fixtures/browser-geography-tract.json.
Original baseline: cargo run --locked -p shield-cli -- hrsa-geography-baseline.
The browser's embedded canonical baseline matches that CLI byte-for-byte.

Queries run locally in WASM without HRSA fetches or uploads. Native wording
that says current means the frozen July 31 cohort, not current October data.
No CMS/provider identity join, whole-county shortage, patient access, staffed
capacity, clinical/payer/funding decision, cost, saving or candidate promotion.
Nine service floors remain null; CMS join and TAXLANE admission remain false.
The native research public-release hold remains distinct from owner-authorized
aggregate browser publication.

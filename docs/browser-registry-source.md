# Frozen primary-care shortage-registry browser source

The registry view replays SHIELD's committed July 31, 2026 HRSA primary-care
HPSA aggregate. It is not a current registry query. The native source loader,
validation, baseline serializer and held pack are unchanged. The browser adds
status selection, not a new shortage or healthcare-service finding.

Rows are designation components. HPSA IDs are counted uniquely within each
status; no deduplicated all-status ID union is established. Do not add status
ID totals. The designation-type, rural-status and component-multiplicity
summaries always describe the fixed Designated cohort, even if another status
is selected. Multiple rural-status IDs remain a distinct multiplicity bucket,
not a new rural class or a forced assignment. No status-by-type/rurality/state
cross-tab or CMS hospital identity is supplied. People can overlap across
designations; no deduplicated population or workforce is established.

The separate quarterly report covers designations as of June 30, 2026, with
July 1 data-as-of context. Its type and rurality partitions retain their own
reported denominator. Cross-vintage reconciliation remains false; this task
computes no delta, trend, percent difference or causal change between files.

## Custody

- Frozen aggregate: `registry-source.json` on Pages, copied byte for byte from
  `data/derived/hrsa-primary-care-hpsa-census-2026-07-31.json`.
- Its aggregate-file hash is in `source-custody.json`; that verifies this
  derivative file, not original publisher bytes.
- Historical CSV capture recorded in that aggregate: 48,303,351 bytes,
  SHA-256 `8ed0007ef82194e44a1ee9086723e42f79e5effe96104cf3104648f2837d2673`.
- Historical quarterly PDF capture: 937,344 bytes,
  SHA-256 `e71178b1c88a6b00a8a4a28780ecbff1f75e39f490ea43a8e5c5c899e211f89c`.
- Those original files are not in the portable bundle and are not reverified or
  reacquired by this work. The publisher CSV/report endpoints are mutable and
  may now return newer releases. They are reference links, not promised frozen
  downloads. Existing source attribution and third-party terms remain intact;
  no publisher content is relabelled as SHIELD original content.

The native baseline's `current_registry` field is preserved for compatibility
and always means this frozen July 31 snapshot, not current October data.
Exports include portable-custody flags describing the absent raw originals.
Queries execute in WASM without fetching HRSA data or uploading inputs.

Native replay: `cargo run --locked -p shield-web --bin shield-browser -- fixtures/browser-registry-designated.json`.
Original baseline: `cargo run --locked -p shield-cli -- hrsa-primary-care-baseline`.
The browser's embedded baseline canonical bytes must match that CLI output.

## Interpretation and authority

Registry counts do not establish a hospital's shortage status, staffing,
appointment/travel access, capacity, quality, outcomes, need, adequate care or
savings. No individual records, rankings, clinical recommendations, designation,
workforce, facility, payer, funding or fiscal decisions are emitted. Native
service floors remain null and TAXLANE admission false. The research held-pack's
public-release flag remains false; browser publication does not promote a
research candidate or grant clinical/fiscal authority.

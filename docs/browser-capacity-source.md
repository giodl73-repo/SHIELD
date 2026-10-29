# Frozen designation-formula capacity source

This view replays the unchanged July 31, 2026 HRSA Designated capacity
aggregate and native validators/baseline/held-pack serializers (WP-011).
Select all formula-bearing designations, area, correctional or policy-excluded
facilities. These are national HPSA-ID formula cohorts, not staffed service
lines, a unique physician census, CMS hospital identities or patient access.
Native current wording means this frozen cohort, not current October data.

The 3,388 formula-bearing IDs comprise 2,838 area and 550 correctional IDs.
The 4,294 other facility designations are excluded by policy: absent FTE,
shortage and need-met values remain null, not zero. Observed zero values among
formula-bearing records stay separate. NP/PA supply is not included. Population
values are designation-recorded sums and may overlap; they are not unique people.
Area/correctional formulas remain separate. The all-formula need-met ratio uses
the native aggregate FTE/(FTE + shortage), not an average of cohort percentages.
It is a designation-formula ratio, not percent patient access or actual need met.

FTE/shortage remain exact integer ten-thousandths; native display strings retain
four decimal places. Native basis-point ratios display two percent decimals.
The area population identity has a signed -1 person-unit residual across one
designation (2,837 exact). Both original tolerance checks cover all 2,838 area
IDs: served within half a person and shortage within 0.01 FTE. Recorded provider
ratio labels, including 1000:0, are shown as source labels without inventing a
quotient or cross-cohort ratio rule. Excluded-population field presence is
reported separately, not filled or treated as capacity coverage.

Download capacity-source.json: exact copy of
data/derived/hrsa-primary-care-designation-capacity-2026-07-31.json.
Its derivative SHA-256 is in source-custody.json. Historical primary records:
CSV 48,303,351 bytes / 8ed0007ef82194e44a1ee9086723e42f79e5effe96104cf3104648f2837d2673;
metadata XLSX 27,297 bytes / 0439dbf209f0b05745e99cc0911e49262b422c564282a48b9030a6175b7ab91b.
Original CSV/XLSX files are not bundled, reacquired or reverified by this work.
Publisher endpoints may return newer releases; they are not promised frozen
downloads. Existing source attribution and third-party terms remain intact.

Native replay: cargo run --locked -p shield-web --bin shield-browser -- fixtures/browser-capacity-excluded.json.
Original baseline: cargo run --locked -p shield-cli -- hrsa-capacity-baseline.
Embedded canonical bytes must match that original CLI. Queries run locally in
WASM with no HRSA request on load and no upload or personal inputs.

Nine service floors remain null, unique-physician supply/CMS capacity/admission
false. No clinical/designation/workforce/facility/payer/funding or fiscal action,
cost, effect or research-candidate promotion. The native research public-release
hold is separate from the owner-authorized aggregate browser publication.

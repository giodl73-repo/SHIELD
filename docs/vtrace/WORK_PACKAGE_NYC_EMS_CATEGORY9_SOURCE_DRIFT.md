# WP-025: NYC EMS Category 9 source-drift monitor

Status: **accepted**

## Objective and result

Determine whether the 387-incident calendar-2025 difference between the direct
Open Data table and Power BI is a localized anomaly, a definition conflict, or
a bounded cross-snapshot drift pattern. Re-query the official Power BI model at
the same month/borough/measure grain and reconcile all 72 cells without
overwriting or blending either source.

The sources share the Citywide Performance Reporting system family and the same
Category 9 definition, but not a refresh timestamp. Open Data was updated at
2026-07-22 18:35:35 UTC. Power BI reports a refresh at 2026-07-27 15:07:31.633
UTC, 419,516.633 seconds later.

Open Data is higher in 54 cells, equal in 18, and lower in zero. Every calendar
month and all six borough labels have a nonzero aggregate difference. Monthly
differences range from 15 to 46 incidents; no individual cell differs by more
than 23. The 72-cell difference sums exactly to 387.

This rejects a one-cell or one-month anomaly. It is consistent with broad
cross-snapshot revision, but it does not prove why records changed. No publisher
revision note or public row-version history was located. Calling either source
wrong would exceed the evidence.

## Precedence and monitoring rule

- Captured official headline: use the later-refresh Power BI exact measure,
  labelled with its refresh timestamp.
- Machine replay: use the documented Open Data API, labelled with its snapshot
  timestamp and two-decimal-percentage-point row precision.
- Never average, splice, or silently overwrite cells across the two snapshots.
- Re-run the complete 72-cell comparison whenever either source refreshes.
- Reopen precedence only if the snapshots converge or the publisher documents
  the revision mechanism.

## Fixed-point review

| Review lens | Disposition |
|---|---|
| Health-system planner | pass — source drift changes custody, not network need or design. |
| Clinician / care delivery | pass — no response or outcome interpretation follows from record revision. |
| Operations / capacity | pass — the 387 records are not relabelled as capacity, demand, or performance change. |
| Health economist | pass — no effect, cost, value, or savings is inferred. |
| Equity / access | pass — borough differences are source drift, not causal disparity findings. |
| Public health / prevention | pass — no morbidity, mortality, or prevention outcome is introduced. |
| Payer / consolidation | pass — no payment or market consequence is inferred. |
| Citation auditor | pass — both complete queries, timestamps, row counts, and hashes have custody. |
| Numeracy checker | pass — 54 + 18 + 0 = 72; monthly and borough deltas each sum to 387. |
| Scope keeper | pass — snapshot drift, error, operations, outcome, candidate, and fiscal claims remain separate. |

No critical or major finding remains.

## Product surfaces

- `data/derived/nyc-ems-category9-source-drift-2025.json`
- `shield nyc-ems-category9-source-drift-baseline`
- `shield nyc-ems-category9-source-drift-held-pack`

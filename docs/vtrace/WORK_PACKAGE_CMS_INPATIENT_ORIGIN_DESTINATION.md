# WP-016: CMS Inpatient Origin-Destination Flow

Status: **accepted**

## Objective

Establish a reproducible 2024 Medicare inpatient origin-destination baseline by
joining beneficiary mailing ZIP and hospital CCN observations from the CMS
Hospital Service Area file to same-year Q4 Provider of Services hospital ZIPs.

## Source custody

| Source | Accepted custody |
|---|---|
| CMS Hospital Service Area 2024 | 1,156,702 rows; 26,951,355 bytes; SHA-256 `1698b7fb8372ebafe126099e2b2406a841411f6eb2d22f50ca9add545bff5aa8` |
| CMS HSA data dictionary | 70,220 bytes; SHA-256 `06535c24d23404b14befa8248800dabd0503232f3166f5a5292e4c05384943d1` |
| CMS HSA methodology | 69,971 bytes; SHA-256 `ddd800c5d4904a30ae2885a5a0c98149edb4badd1a3ec919e007928b521390e6` |
| CMS Q4 2024 QIES Provider of Services | 115,647 rows; 83,978,787 bytes; SHA-256 `25cbd95b347dfc92130d4a4a3bc7c79dc7bbd8fbd1a16fc294029f22e7bfcb09` |

## Grain and validity rules

- Preserve every hospital-CCN/beneficiary-mailing-ZIP pair and retain `*`
  suppression rather than treating it as zero.
- Join hospital location only by exact CCN to the Q4 2024 POS hospital surface.
- Classify case totals only for numeric rows with valid five-digit origin ZIPs.
- Treat cases as discharges or visits, not unique beneficiaries.
- Treat different ZIP as observed cross-ZIP inpatient use only—not a county
  crossing, distance, travel time, emergency visit, reason for travel, or
  access failure.

## Product surfaces

- `data/derived/cms-inpatient-origin-destination-2024.json`
- `shield cms-inpatient-origin-destination-baseline`
- `shield cms-inpatient-origin-destination-held-pack`

## Claim contract and fixed point

Allowed: 2024 Medicare inpatient origin-ZIP/hospital-CCN observations,
suppression residuals, same-year provider identity, observable case totals, and
same- versus different-ZIP classification.

Held: emergency-department destinations, county crossings, road distance,
travel time, reasons for travel, total-population demand, unmet need, adequacy,
candidates, effects, costs, savings, allocation, rates, or public release. The
work package is complete when source, suppression, identity, origin-validity,
case-share, and claim-boundary partitions are executable.

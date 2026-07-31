# Wave: CMS County Emergency-Demand Bridge

Status: **complete**

## Goal and result

Execute accepted WP-015 by turning CMS's 2024 Original Medicare Geographic
Variation file into a bounded county emergency-demand and facility-location
bridge.

- 3,197 unique county rows contain 3,143 numeric ED-use rates and 54 suppressed
  or missing values.
- The national rate is 590.5484 ED visits per 1,000 Original Medicare
  beneficiaries; the unweighted county median is 606.6914.
- Exact POS county FIPS places 5,300 current hospitals in covered demand
  counties and leaves 118 facilities outside that county surface.
- 2,435 demand counties contain a current hospital; 762 do not.
- Among the latter, 708 have numeric demand covering 1,527,795 Original
  Medicare beneficiaries and 906,563 ED visits.

## Boundary and fixed point

The geographic measure is beneficiary residence, not treating-facility
location. County co-location does not observe cross-county travel or access,
and Original Medicare does not represent the total population. Utilization is
not automatically unmet need. Travel/catchments, total-population demand,
adequacy, causal effects, candidates, costs, savings, allocation, and rates
remain held.

## Verification

The feature crate has 36 tests and the workspace has 70. Formatting, strict
clippy, workspace tests, and both compact JSON CLI replays pass. The held pack
is 5,544 UTF-8 bytes without a trailing newline and has SHA-256
`a92ce01a978fc25517b199559fff0201a179a9f473a32f9848be5ca4fdd8aaef`.

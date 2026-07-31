# Wave: HRSA Primary-Care Designation–Component–Geography Bridge

Status: **complete**

## Goal and result

Execute accepted WP-010 by converting the July 31, 2026 HRSA primary-care
registry into a same-vintage geography bridge without collapsing its grain.

- 2,838 area designation IDs expand to 15,524 component rows.
- 2,088 area IDs use Single County components, 586 Census Tracts, and 164
  County Subdivisions; no designation mixes component classes.
- 762 area IDs span multiple components and 155 span multiple county keys.
- 4,844 facility designations remain a separate one-row-per-ID class.
- 7,664 of 7,682 current IDs have an internally consistent common county key.
- All area rows validate; 18 facility IDs remain a visible geography residual:
  17 placeholder keys and one state-prefix inconsistency.

## Boundary and fixed point

A common county key locates a component. It does not turn Census Tract, County
Subdivision, population-group, or facility designations into whole-county
shortage. County co-location is not a CMS facility identity match. Components
and designation populations remain non-additive, and the bridge supplies no
patient access, staffed capacity, adequacy, candidate effect, costs, savings,
allocation, or rates.

## Verification

The feature has sixteen tests and the workspace has 50. Formatting, clippy,
all workspace tests, and both CLI replays pass. The compact fourteen-section
held pack is 5,007 UTF-8 bytes without a trailing newline and has SHA-256
`9c5122e55aa5faf5aa20a7caa4608361b43c9d709403f7625768db96503247d9`.

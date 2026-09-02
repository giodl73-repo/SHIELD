$ErrorActionPreference = 'Stop'

function Assert-Contains {
  param(
    [string]$Path,
    [string]$Needle
  )

  $text = Get-Content -Raw -LiteralPath $Path
  if ($text.IndexOf($Needle, [StringComparison]::Ordinal) -lt 0) {
    throw "Missing expected text in ${Path}: ${Needle}"
  }
}

Assert-Contains 'README.md' 'public access claim boundary'
Assert-Contains 'README.md' 'SHIELD-PF-05'
Assert-Contains 'README.md' 'aggregate-only scoping evidence'

Assert-Contains 'docs/adoption/README.md' 'public access claim boundary'
Assert-Contains 'docs/adoption/README.md' 'Health-System Planner'
Assert-Contains 'docs/adoption/README.md' 'Payer & Consolidation Realist'

Assert-Contains 'docs/adoption/aggregate-adaptation-worksheet.md' 'Public Access Claim Boundary'
Assert-Contains 'docs/adoption/aggregate-adaptation-worksheet.md' 'SHIELD-PF-05'
Assert-Contains 'docs/adoption/aggregate-adaptation-worksheet.md' 'parliament and editorial dispositions'

Assert-Contains 'docs/adoption/public-access-claim-boundary.md' '`SHIELD-PF-05`'
Assert-Contains 'docs/adoption/public-access-claim-boundary.md' 'artifact path and regeneration command'
Assert-Contains 'docs/adoption/public-access-claim-boundary.md' 'If any field is missing'
Assert-Contains 'docs/adoption/public-access-claim-boundary.md' 'Health-System Planner,'

Assert-Contains 'docs/vtrace/VERIFICATION.md' 'public access claim boundary'
Assert-Contains 'docs/vtrace/VERIFICATION.md' 'SHIELD-PF-05'
Assert-Contains 'docs/vtrace/VERIFICATION.md' 'REQ-009..011 remain'

Assert-Contains '.roles/ROLE.md' '## PITFALL gates'
Assert-Contains '.roles/ROLE.md' '`SHIELD-PF-05`'
Assert-Contains '.roles/ROLE.md' 'Health-System Planner; Clinician / Care-Delivery Lead; Operations & Capacity Officer; Health Economist; Equity & Access Advocate; Public-Health & Prevention Advocate; Payer & Consolidation Realist; Citation Auditor; Scope Keeper; Numeracy Checker'

Assert-Contains '.pitfall/shield-pitfalls.md' '**Status:** MITIGATED'
Assert-Contains '.pitfall/shield-pitfalls.md' 'tests/check-public-access-claim-boundary.ps1'
Assert-Contains '.pitfall/shield-invariants.md' 'SHIELD-I-06'
Assert-Contains '.pitfall/shield-invariants.md' 'Public Access Claims Require Release Evidence'
Assert-Contains '.pitfall/shield-invariants.md' 'SHIELD-PF-05'

Write-Host 'SHIELD public access claim boundary check passed.'

## Summary

Describe the change and the SHIELD surface it affects.

## Privacy And Claim Boundary

- [ ] No patient records or individual medical advice are introduced.
- [ ] Evidence labels, aggregate scope, and source custody are preserved.
- [ ] No clinical, licensing, accreditation, Certificate-of-Need, payer, endorsement, ROI, or validation claim is introduced unless explicitly source-backed and reviewed.
- [ ] Any changed public claim has a traceable source path or is marked held/source-needed/confidence-limited.

## Validation

```powershell
git diff --check -- README.md CONTRIBUTING.md docs .github
```

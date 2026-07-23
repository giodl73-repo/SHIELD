# Contributing

Keep SHIELD aggregate, evidence-labelled, and explicit about the difference
between analysis and clinical, licensing, payer, or individual decisions.

Useful public contributions include aggregate source inventories, access and
referral evidence, workforce or affordability review, privacy review, and safer
public language. For aggregate adaptations, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p shield-cli -- --help
```

Do not commit restricted datasets, credentials, local build state, patient
records, personal records, or uncited public claims.

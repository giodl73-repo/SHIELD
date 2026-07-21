# Contributing

Keep SHIELD scale-aware, evidence-labelled, and explicit about the difference between analysis and operational, regulatory, or individual decisions.

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p shield-cli -- --help
```

Do not commit restricted datasets, credentials, local build state, personal records, or uncited public claims.

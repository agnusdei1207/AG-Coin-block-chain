# AG Coin Blockchain

Minimal blockchain learning project based on the Polkadot SDK solochain template.

## Docker Compose only

All Rust builds, checks, and tests must run inside Docker Compose. Do not run
`cargo build`, `cargo check`, or `cargo test` directly on the host.

```bash
docker compose build node
docker compose run --rm test
docker compose up node
```

The Compose file uses the current Compose Specification, so it intentionally
does not contain the obsolete top-level `version` field.

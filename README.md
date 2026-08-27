# AG Coin Blockchain

Minimal blockchain learning project based on the Polkadot SDK solochain template.

## MVP scope

This repository is deliberately only the smallest practical blockchain
skeleton for learning. AG Coin itself is **not implemented yet**. Until its
requirements are understood, do not add token issuance, custom economics,
wallets, governance, a frontend, or unrelated infrastructure.

Every future change must preserve these priorities:

1. Simplicity
2. Minimality
3. Necessity
4. Readability

**Simple is best.** Add only what the current learning milestone requires,
keep module boundaries obvious, and remove accidental complexity before adding
features.

## Docker Compose only

All Rust builds, checks, and tests must run inside Docker Compose. Do not run
`cargo build`, `cargo check`, or `cargo test` directly on the host.

```bash
docker compose run --rm build
docker compose run --rm test
docker compose up node
```

The Compose file uses the current Compose Specification, so it intentionally
does not contain the obsolete top-level `version` field.

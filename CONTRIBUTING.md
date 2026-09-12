# Contributing to vf-privacy-kit

Thanks for considering a contribution to a security-critical kit. The bar is deliberately high — every VF product depends on this code.

## Before you open a PR

1. **Read the [threat model](./docs/threat-model.md).** Every change is evaluated against it. If your PR expands the threat surface, say so explicitly in the description.
2. **Read the [API reference](./docs/api-reference.md).** New public API surface requires an accompanying doc update in the same PR.
3. **Check the [style guide](./docs/style.md)** for Rust + TypeScript conventions.

## Contributor licence

External contributions are accepted under the [Apache-2.0 licence](./LICENSE). Sign the DCO in each commit:

```
git commit -s -m "your message"
```

This adds a `Signed-off-by:` trailer. See [developercertificate.org](https://developercertificate.org/) for what you're certifying.

## Development workflow

```bash
# Rust
cd crates/vf-privacy-kit
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
cargo test --features slow-tests   # runs the fuzzy timing checks

# TypeScript (WASM-backed)
cd packages/vf-privacy-kit
pnpm install
pnpm test
pnpm build   # rebuilds WASM from the Rust source

# Cross-language integration tests
cd tests/cross-lang
./run.sh   # exercises the same test vectors through Rust + TS + Swift + Kotlin
```

## Reviews

Every PR requires:

- ✅ CI green on all four target platforms (Linux, macOS, Windows, WASM).
- ✅ At least one review from a maintainer.
- ✅ For crypto changes: a second review from a maintainer with cryptographic expertise + explicit sign-off from the security lead.
- ✅ For new public API: a doc-side review confirming the API reference is updated.

Turnaround: we aim for first response within 3 working days, review completion within 7. Larger changes take longer — say hi in an issue first.

## What we welcome

- Fuzz + property-based tests exercising the primitive under adversarial inputs.
- Documentation improvements (examples, threat-model refinements, review of counterexamples).
- Cross-platform bug fixes.
- Performance improvements that don't compromise the security properties.

## What we're unlikely to accept

- **New crypto primitives.** We use well-studied, standardised algorithms (X25519, Ed25519, BLAKE3, age). Proposing to add something like a novel ratchet or hybrid PQ-scheme requires an ADR + threat-model refresh + security-lead approval before code review.
- **Additional dependencies.** Every crate/npm dep is a supply-chain concern. Prefer stdlib.
- **Behaviour changes that add opt-out privacy toggles.** The kit's design principle is "privacy by architecture" — no per-user privacy setting. If you think you need a toggle, open an issue first.

## Security disclosures

Do **not** open public issues for vulnerabilities. See [SECURITY.md](./SECURITY.md).

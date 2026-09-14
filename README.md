<div align="center">

# 🔐  `@vf/privacy-kit`

**One primitive. Every VF product uses it.**

Sealed identity, sealed transport, sealed offline data packs — the shared
crypto and networking layer that makes every VF Empire product safe by
default. No product rolls its own crypto.

[![License: FSL-1.1-ALv2](https://img.shields.io/badge/license-FSL--1.1--ALv2-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-lifting_from_vfmail--desktop-yellow.svg)](https://github.com/vfempire-hq/vfmail-desktop/tree/main/src-tauri/src)
[![security-scan](https://github.com/vfempire-hq/vf-privacy-kit/actions/workflows/security-scan.yml/badge.svg)](https://github.com/vfempire-hq/vf-privacy-kit/actions/workflows/security-scan.yml)

</div>

---

## Why this exists

If ten products each write their own crypto, you get ten different bugs. If
they share one implementation, you get one carefully-audited primitive that
every product benefits from.

**LAW inside VF Empire:** every product that transmits, stores, or exposes
any user data uses this kit. No per-app crypto, no per-app transport, no
per-app data store. Enforced at review.

## What it gives you

- **Sealed identity**: age (X25519) encryption keypair + Ed25519 signing
  keypair. Private half wrapped by a vault password, never leaves the
  device. Public half published to `/.well-known/vfmail-keys/<user>.json`
  on the user's own domain.
- **Sealed messages**: age-seal a body to one or many recipients. Ed25519
  detached signature verifies the sender without exposing anything else.
- **Sealed Channel transport**: envelope-hiding poll (Phase 1) or WebSocket
  (Phase 2) that carries only `{ recipient_id: sha256(pubkey), sealed_blob }`.
  The relay server does NOT know sender email, recipient email, subject,
  or timestamp linked to identity.
- **Sealed offline data packs**: age-encrypted regional data bundles
  (offline maps, POI databases, model weights). Client verifies signature,
  decrypts, uses locally. No per-query round-trip.
- **Auth-gated public-key directory**: only holders of a valid JMAP session
  can look up other users' pubkeys, defeating outsider enumeration.

## What it does NOT give you

- Bulletproof anonymity at the IP layer. Phase 3 (Tor/mixnet) later.
- Zero-knowledge identity proof. Sealed Channel is authenticated, not
  anonymous — the recipient's *client* knows the sender, the *server* does
  not.
- Business logic. If you need webhook verification for Stripe or Google,
  keep that per-product.

---

## Status: skeleton (Wave 1 refactor in progress)

The actual working implementation currently lives in
[`vfmail-desktop`](https://github.com/vfempire-hq/vfmail-desktop) —
specifically:

```
vfmail-desktop/src-tauri/src/
├── identity.rs        ← lift to vf-privacy-kit/crates/identity
├── mail_crypto.rs     ← lift to vf-privacy-kit/crates/sealed-mail
├── channel.rs         ← lift to vf-privacy-kit/crates/channel
└── keydir.rs          ← lift to vf-privacy-kit/crates/keydir
```

Wave 1 lifts those files into this repo as a Rust workspace, adds a
TypeScript wrapper via WASM + native npm, publishes as
`@vf/privacy-kit` on npm and `vf-privacy-kit` on crates.io.

Wave 2: FUEL·IT (`@vfempire-hq/fuelit`) migrates NAV·IT Ed25519 origin
signing, offline-pack signature verify, and rolling-HMAC tokens off its
local implementation onto `@vf/privacy-kit`.

Wave 3: Swift Package + Android AAR builds for iOS/Android FUEL·IT
native modules to consume via SPM + Maven Central.

## Planned surfaces

- npm `@vf/privacy-kit` — TypeScript, Node + browser + Bun
- crates.io `vf-privacy-kit` — Rust
- Swift Package Manager `VfPrivacyKit` — iOS/macOS
- Maven Central `com.vfempire:privacy-kit` — Android/JVM

All three ports track the same version number and expose the same API
shape. Reference implementation = Rust. Others via bindings.

---

## Licence

Licensed under the **[Functional Source License 1.1 (ALv2)](LICENSE)** — replaces the earlier AGPL-3.0 + commercial dual-licence posture as of 2026-09-14.

FSL is source-available: **free** for any use that isn't a Competing Use (internal use, non-commercial education, non-commercial research, professional services provided to FSL-licensed users). It **restricts** redistribution as a commercial product that offers substantially similar functionality to `vf-privacy-kit`. And it **auto-converts each release to Apache 2.0 after two years** — every version becomes fully open source on its second birthday, guaranteed by the licence itself.

**Why FSL for a shared crypto substrate:** the whole point of `vf-privacy-kit` is that every VF Empire product inherits the same sealed-identity + sealed-transport + offline-data-pack primitives. FSL protects the substrate from being lifted verbatim into a competing product suite, while still letting security auditors, contributors, and downstream open-source projects use it freely. Every VF product carries this crate under the same FSL grant.

VF Empire in-house products (VF Mail, FileIT, Guardian Shield, VF Wallet, VF Legacy, VF Home, SnapIT) exercise the "Permitted Purpose" grant by default — they are not Competing Uses because they are all *from* VF Empire.

For questions about specific use cases — commercial integration, embedding in your own product, custom SLAs — write to `licensing@vfempire.com`. Full FSL text in [LICENSE](LICENSE); spec at https://fsl.software.

---

## Contributing

Not open for external contributions during Wave 1 lift. Once the API
stabilises, contribution guidelines land here. Report vulnerabilities to
`security@vfempire.com`.

---

<div align="center">
<sub>© 2026 VF Empire Corp Ltd · Malta C 94160</sub>
</div>

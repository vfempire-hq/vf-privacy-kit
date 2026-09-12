# vf-privacy-kit — threat model

STRIDE-shaped analysis of the shared kit. This document describes what the kit does and does not defend against; every downstream product inherits these properties and adds its own product-specific threat model on top.

## Adversary model

- **A1 — nation-state passive observer.** Sees TLS metadata: source IP, destination hostname, request timing + size. Cannot break TLS 1.3.
- **A2 — hostile server operator.** Has root on the server the kit's channel traverses. Can read any bytes on disk + in RAM.
- **A3 — hostile CDN / MITM.** Has a valid TLS cert for one of our hostnames. Can substitute arbitrary bytes into responses.
- **A4 — device forensic examiner.** Has physical access to a locked, encrypted device. Trying to recover key material from the disk image.
- **A5 — running-app debugger.** Has attached a debugger or memory-inspection tool to a running app process (usually a compromised host or a jailbroken device with the user's cooperation lost).

## What the kit defends against

### Sealed identity (module: `identity`)

| Threat | Mitigation |
|--------|------------|
| A2 reads the identity file on disk | Only `IdentityPublic` is stored; the secret is derived from the user's seed, which is not persisted by the kit (product decides whether to store it, and if so, in a wipe-registered location). |
| A3 replaces a user's public identity with a substitute | Identity hashes are deterministic + user-verified out-of-band (short `vfid_` display strings). Substitution fails at handshake. |
| A5 dumps process memory | Secret handles are wrapped in `Zeroizing`. Best-effort — a debugger attached during a decrypt operation can still see cleartext for its duration. |

### Sealed transport (module: `channel`)

| Threat | Mitigation |
|--------|------------|
| A2 reads message bodies flowing through their server | `sealed_body` is age-encrypted to the recipient's X25519 public key. Server does not have the private key. |
| A2 correlates two envelopes to the same recipient | Per-envelope 128-bit random salt mixed into the routing hash → same-recipient envelopes produce different routing hashes. |
| A3 injects a forged envelope claiming to come from a legitimate sender | Every envelope is Ed25519-signed by the sender. Signature verification is mandatory on receipt. |
| A1 sees traffic patterns (envelope sizes + timings) | **Not fully mitigated.** Documented residual risk. Products should pad envelopes to 4 KB power-of-2 sizes + jitter response times if the threat model demands. |

### Sealed offline packs (module: `pack`)

| Threat | Mitigation |
|--------|------------|
| A3 substitutes a hostile pack for a legitimate one | Every pack is Ed25519-signed by the pack-builder's key. Client verifies signature before load. |
| A3 substitutes an OLD legitimate pack for a NEW one (rollback attack) | Pack manifests include monotonic version + built-at timestamp. Clients refuse packs older than the last-installed version by policy. |
| A2 with pack-signing-key access forges packs | Pack signing key held in HSM (Yubikey), rotated annually. Compromise triggers emergency key-rotation runbook + client update. |

### Rolling tokens (module: `token`)

| Threat | Mitigation |
|--------|------------|
| A2 correlates two token-authenticated requests to the same user | Tokens don't identify users — they're derived only from `(current_minute, symmetric_key)`. A2 sees the same token across all users during a minute. |
| Attacker replays a captured token | Token accepted only for ±2-minute rolling window; replay past the window fails. |
| Attacker floods with valid tokens | Server rate-limits per token — an attacker cannot amplify past the per-minute cap without minting new tokens (needs the symmetric key). |

### Panic-wipe (module: `wipe`)

| Threat | Mitigation |
|--------|------------|
| A4 examines the disk after a user triggers wipe | Every kit-registered target is zeroized before unlink. On sensible filesystems, unlinked blocks are marked free + overwritten on next allocation. |
| A4 recovers from filesystem journal | **Not fully mitigated.** Log-structured filesystems (APFS, F2FS) may retain deleted content until compaction. Documented residual risk. |
| A4 recovers from iCloud/Google Drive backup | **Not mitigated.** Products should surface a "disable app backup at OS level" recommendation in their privacy model. |

## What the kit does NOT defend against

- **A1 traffic metadata.** If a nation-state agency can watch a user's ISP, they see that the user contacted our servers. Use Tor if that matters.
- **A5 during active decrypt.** A debugger attached to a running app can see cleartext during the moment of decrypt. Sandbox integrity is the OS's job; the kit cannot save you from a compromised host.
- **Novel cryptographic breaks.** The kit uses well-studied primitives (X25519, Ed25519, age, BLAKE3). If any of those are broken, the kit is broken. We update within 30 days of a credible break.

## Residual risks by module

| Module | Residual risks | Downstream product must document |
|--------|----------------|----------------------------------|
| `identity` | Seed backup is the user's responsibility. If they lose it, keys are unrecoverable. | Yes — flag in product's UI. |
| `channel`  | TLS metadata visible to A1; envelope-size traffic-analysis by A1 not fully mitigated | Yes. |
| `pack`     | Old-legitimate rollback beyond product's version-refuse policy; iCloud/backup residuals | Yes. |
| `token`    | If the symmetric key is exfiltrated, an attacker mints unlimited tokens for that hour. Detected by anomaly rate-limiting. | Optional. |
| `wipe`     | Filesystem journals + off-device backups + running-app debug | Yes. |

## Verification

Downstream products should reference this document in their own threat models, then add product-specific analysis on top. Every publicly-shipping VF product carries a threat-model file at `docs/security/threat-model.md` that names this one as its foundation.

## Change log

- **1.0 (2026-09-12):** initial version, published alongside the kit skeleton.

//! # vf-privacy-kit
//!
//! Shared crypto + sealed-transport primitives for every VF Empire product.
//!
//! ## Modules
//!
//! **Lifted from VF Mail (Wave 1, this session):**
//! - [`crypto`]     — Argon2id → XChaCha20-Poly1305 KEK derivation + master-key wrap.
//! - [`vault`]      — On-disk encrypted vault structs (KdfParams, WrappedMaster, Vault).
//! - [`identity`]   — Sealed identity: age (X25519) + Ed25519 signing keypair.
//! - [`mail_crypto`] — Seal + open sealed mail bodies with detached signature.
//! - [`keydir`]     — Authenticated lookup of published pubkeys.
//! - [`channel`]    — Sealed-channel client (envelope-hiding transport).
//!
//! **FUEL·IT session stubs (Wave 2 lift target):**
//! - [`pack`]  — Ed25519-signed sealed offline data packs.
//! - [`token`] — Rolling per-minute HMAC anti-abuse tokens.
//! - [`wipe`]  — Panic-wipe: deterministic destroy-all-key-material primitive.
//!
//! ## Threat model
//!
//! See [`docs/threat-model.md`](../../docs/threat-model.md).

#![deny(unsafe_code)]
// missing_docs is aspirational — the Wave 1 lift retains the docs it had
// at source; Wave 2 sweep will add per-item rustdoc.
#![allow(missing_docs)]

pub mod crypto;
pub mod vault;
pub mod identity;
pub mod mail_crypto;
pub mod keydir;
pub mod channel;

// FUEL·IT session stubs — Wave 2 lift.
pub mod pack;
pub mod token;
pub mod wipe;

/// The library version, useful for wire-compatibility checks.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

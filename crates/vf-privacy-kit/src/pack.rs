//! Sealed offline data packs.
//!
//! An Ed25519-signed archive format for shipping bulk data (map tiles,
//! routing graphs, geocoding indexes, mail bundles) that the client
//! verifies at load. A hostile CDN cannot forge a pack; a compromised
//! network path cannot swap one pack for another.
//!
//! Used by NAV·IT (routing graphs, PMTiles, Photon indexes), VF Mail
//! (batch bundle downloads), and any future product that ships bulk data.
//!
//! ## Wire format
//!
//! ```text
//! magic:   "VFPK"                            [4 bytes]
//! version: u32 le                            [4 bytes]
//! manifest_length: u32 le                    [4 bytes]
//! manifest: JSON, canonicalised              [manifest_length bytes]
//! signature: Ed25519(manifest || body_hash)  [64 bytes]
//! body: content-addressable chunk stream     [rest]
//! ```
//!
//! ## Status
//!
//! **Stub.** FUEL·IT session's Wave 2 migration will populate this,
//! informed by the FUEL·IT reference implementation at
//! `services/navigator/src/offline-packs.ts` +
//! `clients/ios/FuelIT/Nav/OfflinePackManager.swift`.

use serde::{Deserialize, Serialize};

/// The manifest of a sealed pack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackManifest {
    /// Product this pack belongs to (`nav-it`, `vfmail`, ...).
    pub product: String,
    /// Region or scope of the pack (e.g. `"DE"`, `"planet"`).
    pub region: String,
    /// Version string, monotonically increasing.
    pub version: String,
    /// Total size of the body in bytes.
    pub body_size: u64,
    /// BLAKE3 hash of the body — the whole file is content-addressable.
    pub body_hash: [u8; 32],
    /// When this pack was signed, RFC 3339 datetime.
    pub built_at: String,
    /// Base64 fingerprint of the signing key.
    pub signing_key_fingerprint: String,
}

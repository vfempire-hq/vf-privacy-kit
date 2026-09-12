//! Panic-wipe.
//!
//! Deterministic destroy-all-key-material primitive that leaves no
//! forensic trace on the device. Bound to a user gesture (triple-tap-hold
//! on iOS/Android, keybinding on desktop) that clears every persistent
//! store the kit knows about.
//!
//! ## What gets wiped
//!
//! - Every key file on disk with a `.vfkey` extension inside the app's
//!   sandbox (best-effort — respects platform sandbox boundaries).
//! - The in-memory key cache (zeroized).
//! - Any DataStore/UserDefaults/SharedPreferences key registered via
//!   [`register_wipe_target`].
//!
//! ## What CANNOT be wiped
//!
//! - iCloud/Google-Drive backups of app data. Users should disable app
//!   backup at OS level if wipe integrity matters to their threat model.
//! - Journal entries in filesystem log-structured storage (APFS, F2FS).
//!   The OS may recover a deleted file until its blocks are overwritten.
//! - Off-device backups the app itself made to a cloud sync target.
//!
//! Callers must document these residuals in their product-level privacy
//! model.

// Re-export `zeroize::Zeroizing` so callers can use it for secret buffers
// without adding zeroize as a direct dep.
pub use zeroize::Zeroizing;
pub use zeroize::Zeroize;

/// Register a path or store the kit should include in a panic-wipe.
///
/// Called during app boot for every persistent store that contains
/// key material or user-secret data.
///
/// ## Status
///
/// **Stub.**
pub fn register_wipe_target(_scheme: &str, _handle: &str) {
    // TODO: append to a global registry.
}

/// Execute the wipe. Returns the number of targets that were successfully
/// zeroized. Best-effort; individual failures are logged, not returned as
/// errors, because a partial wipe is still better than none in a
/// duress scenario.
///
/// ## Status
///
/// **Stub.**
pub fn panic_wipe() -> usize {
    0
}

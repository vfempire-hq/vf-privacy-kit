//! Rolling per-minute HMAC anti-abuse tokens.
//!
//! Some VF surfaces (NAV·IT routing, sealed-channel writes, anonymous
//! contact forms) accept requests without user identification but still
//! need rate-limiting. IPs are stripped upstream; we can't rate-limit by
//! IP. Rolling HMAC tokens fill the gap:
//!
//! - Client derives a token via `hmac(current_minute, symmetric_key)`.
//! - Server accepts tokens for the ±2-minute window around now.
//! - Symmetric key rotates hourly; the previous key is accepted for a
//!   5-minute overlap.
//! - Server rate-limits by token, not by IP.
//!
//! An attacker cannot amplify past their per-minute quota without minting
//! new tokens (which requires the key), and they cannot replay past
//! minutes (window is bounded).
//!
//! ## Status
//!
//! **Stub.** FUEL·IT's Wave 2 migration lifts the reference
//! implementation from `services/navigator/src/auth.ts`.

/// Derive the token for a given (minute, key). Constant-time-safe.
pub fn derive(_minute: u64, _key: &[u8]) -> [u8; 32] {
    // TODO(fuelit-lift): HMAC-BLAKE3(key, minute_le_bytes)
    [0u8; 32]
}

/// Verify a candidate token against the rolling window. Returns `true`
/// iff any of (now-2, now-1, now, now+1, now+2) minutes with either the
/// current or previous key produces a match.
pub fn verify(_candidate: &[u8; 32], _now_minute: u64, _current_key: &[u8], _previous_key: Option<&[u8]>) -> bool {
    // TODO(fuelit-lift): constant-time comparison across the window.
    false
}

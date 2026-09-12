/**
 * Rolling per-minute HMAC anti-abuse tokens — see the Rust crate's
 * `token` module for the canonical spec.
 */

/** Derive the token for a given (minute, key). */
export function derive(_minute: number, _key: Uint8Array): Uint8Array {
  // TODO(fuelit-lift): call the WASM export.
  return new Uint8Array(32);
}

/**
 * Verify a candidate token against the rolling window.
 *
 * Returns true iff any of (now-2, now-1, now, now+1, now+2) minutes with
 * either the current or previous key produces a match. Constant-time-safe.
 */
export function verify(
  _candidate: Uint8Array,
  _nowMinute: number,
  _currentKey: Uint8Array,
  _previousKey?: Uint8Array,
): boolean {
  // TODO(fuelit-lift)
  return false;
}

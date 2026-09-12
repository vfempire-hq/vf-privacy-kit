/**
 * Sealed transport channel — see the Rust crate's `channel` module for
 * the canonical spec.
 */

export interface Envelope {
  /** One-way hash of the recipient's identity. */
  routingHash: Uint8Array;
  /** Per-envelope random salt. */
  salt: Uint8Array;
  /** age-encrypted payload. */
  sealedBody: Uint8Array;
  /** Sender's Ed25519 signature over `sealedBody || salt`. */
  signature: Uint8Array;
}

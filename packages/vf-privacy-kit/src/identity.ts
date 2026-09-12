/**
 * Sealed identity — see the Rust crate's `identity` module for the
 * canonical spec. This file is the TypeScript-facing API; the actual
 * cryptography runs in WASM.
 */

export interface IdentityPublic {
  /** X25519 public key for encryption (age-compatible). */
  encPk: Uint8Array;
  /** Ed25519 public key for signing. */
  signPk: Uint8Array;
  /** Stable identity hash = BLAKE3(encPk || signPk). */
  idHash: Uint8Array;
}

/**
 * Derive an identity deterministically from a 32-byte seed.
 *
 * The seed is what the user backs up. From it, all keys are re-derivable
 * offline forever. Do NOT persist the seed anywhere the kit does not
 * manage — use [`registerWipeTarget`](./wipe.js) to bind persistence to
 * the panic-wipe.
 *
 * @param seed exactly 32 bytes of entropy
 * @throws {@link VFPrivacyError} if seed is not 32 bytes
 */
export async function fromSeed(_seed: Uint8Array): Promise<{
  publicIdentity: IdentityPublic;
  secretHandle: unknown;
}> {
  // TODO(vf-mail-lift): call the WASM export.
  throw new Error('identity.fromSeed: not yet implemented (Wave 1)');
}

/**
 * Encode an identity as a short human-readable string.
 *
 * Format `vfid_<base32-crockford of idHash>` — exactly 40 chars.
 */
export function toDisplay(_id: IdentityPublic): string {
  // TODO(vf-mail-lift)
  return 'vfid_TODO';
}

/**
 * Sealed offline data packs — see the Rust crate's `pack` module for the
 * canonical spec.
 */

export interface PackManifest {
  product: string;
  region: string;
  version: string;
  bodySize: number;
  bodyHash: Uint8Array;
  builtAt: string;                    // RFC 3339
  signingKeyFingerprint: string;
}

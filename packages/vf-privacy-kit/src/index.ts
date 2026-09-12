/**
 * # @vf/privacy-kit
 *
 * TypeScript API for the vf-privacy-kit primitives. WASM-backed under the
 * hood — the reference implementation lives in the sister Rust crate
 * `vf-privacy-kit` and is compiled to WebAssembly for consumption from web
 * / Node / React Native.
 *
 * ## Modules
 *
 * - `./identity` — sealed identity
 * - `./channel`  — sealed transport
 * - `./pack`     — sealed offline data packs
 * - `./token`    — rolling anti-abuse tokens
 * - `./wipe`     — panic-wipe
 *
 * ## Status
 *
 * Pre-1.0. `identity` and `channel` land first (Wave 1), from VF Mail's
 * production code. `pack` and `token` follow (Wave 2), from NAV·IT.
 */

export * from './identity.js';
export * from './channel.js';
export * from './pack.js';
export * from './token.js';
export * from './wipe.js';

/** Kit version, useful for wire-compatibility checks. */
export const VERSION = '0.1.0';

/**
 * Errors raised by the kit. Kept coarse deliberately — treat any error as
 * "the operation failed, do not retry with the same input" and log details
 * via your own telemetry.
 */
export class VFPrivacyError extends Error {
  constructor(
    message: string,
    public readonly code: 'verification' | 'format' | 'input' | 'unsupported',
  ) {
    super(message);
    this.name = 'VFPrivacyError';
  }
}

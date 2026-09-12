/**
 * Panic-wipe — see the Rust crate's `wipe` module for the full doc.
 */

/**
 * Register a persistent store the kit should include in a panic-wipe.
 * Called during app boot for every store containing key material.
 */
export function registerWipeTarget(_scheme: string, _handle: string): void {
  // TODO: append to the global registry.
}

/**
 * Execute the wipe. Returns the number of targets successfully zeroized.
 * Best-effort — partial wipe is better than none in a duress scenario.
 */
export function panicWipe(): number {
  // TODO
  return 0;
}

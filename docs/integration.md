# Integrating vf-privacy-kit

How to consume the kit from each of the four target languages.

## Rust — `cargo`

```toml
# Cargo.toml
[dependencies]
vf-privacy-kit = "0.1"
```

```rust
use vf_privacy_kit::{identity, channel};

let seed = [0u8; 32]; // in a real app: random bytes from OsRng, then backed up
let (pk, sk) = identity::Identity::from_seed(&seed);

// ... later ...
let envelope = channel::Channel::seal(&recipient_pk, &sk, b"hello");
```

## TypeScript — `pnpm add @vf/privacy-kit`

```typescript
import { identity, channel } from '@vf/privacy-kit';

const seed = crypto.getRandomValues(new Uint8Array(32));
const { publicIdentity, secretHandle } = await identity.fromSeed(seed);
```

WASM is loaded lazily on first use. In Node, the WASM is bundled; in browsers, it's loaded from the package's `wasm/` directory.

## Swift — Swift Package Manager

Once the Swift artefact ships (planned Q4 2026):

```swift
// Package.swift
dependencies: [
    .package(url: "https://github.com/vfempire-hq/vf-privacy-kit-swift", from: "0.1.0"),
]
```

```swift
import VFPrivacyKit

let seed = Data((0..<32).map { _ in UInt8.random(in: 0...255) })
let identity = try Identity.fromSeed(seed)
```

## Kotlin / Android — Gradle

Once the Android AAR ships (planned Q4 2026):

```kotlin
// build.gradle.kts
dependencies {
    implementation("com.vfempire:vf-privacy-kit:0.1.0")
}
```

```kotlin
import com.vfempire.privacy.Identity

val seed = ByteArray(32).also { SecureRandom().nextBytes(it) }
val identity = Identity.fromSeed(seed)
```

## Which downstream products depend on which modules

Enforced at review — every product PR that adds crypto goes through this table:

| Product | `identity` | `channel` | `pack` | `token` | `wipe` |
|---------|:---------:|:---------:|:------:|:-------:|:------:|
| VF Mail Desktop | ✅ Wave 1 | ✅ Wave 1 | ⏳ Wave 2 | — | ⏳ Wave 2 |
| VF Mail iOS/Android | ⏳ Wave 3 | ⏳ Wave 3 | ⏳ Wave 2 | — | ⏳ Wave 3 |
| FuelIT + NAV·IT | — | — | ⏳ Wave 2 | ⏳ Wave 2 | — |
| MapIT (R·03) | — | — | ⏳ Wave 2 | ⏳ Wave 2 | — |
| Guardian Shield (planned) | ⏳ | — | — | — | ⏳ |

Legend: ✅ shipped · ⏳ planned · — not applicable to this product.

## Enforcement

CI on every dependent repo runs `.github/workflows/nav-banned-deps.yml`-style check that greps for any of these patterns and fails:

- New usages of `hmac::*`, `sha2::*`, `ring::*`, `sodiumoxide::*` outside `vf-privacy-kit` itself
- Direct calls to platform crypto: `CryptoKit.*`, `java.security.Signature`, `webcrypto.subtle.*` — should route through the kit's wrappers
- Custom HMAC / signature implementations

Explicit exceptions (Stripe HMAC in FuelIT, per ADR-0005) get an allow-list entry with a comment.

// VFPrivacyKit — Swift binding for vf-privacy-kit.
//
// This file is a stub; the shipping build replaces it with auto-generated
// bindings from `uniffi-rs`. Consumers should import `VFPrivacyKit` and
// use the exported types (Identity, Channel, Pack, Token, Wipe).

import Foundation

public enum VFPrivacyKit {
    public static let version = "0.1.0"

    public enum Error: Swift.Error {
        case verification
        case format
        case input
        case unsupported
    }
}

// Placeholder types matching the Rust crate's public surface. Replaced
// by uniffi-generated bindings at build time.
public struct IdentityPublic: Codable, Hashable {
    public let encPk: Data
    public let signPk: Data
    public let idHash: Data

    public init(encPk: Data, signPk: Data, idHash: Data) {
        self.encPk = encPk
        self.signPk = signPk
        self.idHash = idHash
    }
}

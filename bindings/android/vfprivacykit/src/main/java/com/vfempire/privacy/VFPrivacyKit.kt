package com.vfempire.privacy

/**
 * Kotlin binding for vf-privacy-kit. Stubbed until the uniffi generator
 * produces the real bindings from the Rust source.
 */
object VFPrivacyKit {
    const val VERSION = "0.1.0"

    sealed class Error(message: String) : Exception(message) {
        object Verification : Error("cryptographic verification failed")
        object Format       : Error("malformed wire format")
        object Input        : Error("invalid input")
        object Unsupported  : Error("feature not compiled in this build")
    }
}

data class IdentityPublic(
    val encPk: ByteArray,
    val signPk: ByteArray,
    val idHash: ByteArray,
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is IdentityPublic) return false
        return encPk.contentEquals(other.encPk) &&
               signPk.contentEquals(other.signPk) &&
               idHash.contentEquals(other.idHash)
    }

    override fun hashCode(): Int {
        var result = encPk.contentHashCode()
        result = 31 * result + signPk.contentHashCode()
        result = 31 * result + idHash.contentHashCode()
        return result
    }
}

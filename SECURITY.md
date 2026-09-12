# Security policy — vf-privacy-kit

Since every VF product depends on this kit, a vulnerability here is a vulnerability in the whole product line. Please report responsibly.

## Reporting

**Please do NOT open a public issue.** Report via:

- Email: `security@vfempire.com`
- PGP: [vfempire.com/.well-known/pgp.txt](https://vfempire.com/.well-known/pgp.txt)
- VF Mail: `security@vfempire.com` (end-to-end encrypted if you're on VF Mail)

## Scope

**In scope:**

- Cryptographic weaknesses in the sealed-identity, sealed-transport, or sealed-pack constructions.
- Side-channel leaks (timing, memory pressure, cache) in any implementation.
- Panic-wipe bypasses that leave key material recoverable.
- Signature-verification bypasses.
- Any way to make a downstream product accept forged data claimed to come from a legitimate origin.

**Out of scope:**

- Denial-of-service (rate-limit exhaustion) — these are documented residual risks, tracked as reliability not security issues.
- Physical attacks on unlocked devices — the kit protects against remote adversaries and post-seizure forensics on locked devices; it does not defend a running app whose process memory is under a debugger.

## Bounty

We pay bounties for confirmed privacy or crypto vulnerabilities:

| Severity | Payout (EUR) | Example |
|----------|--------------|---------|
| Critical | 10 000 – 25 000 | Practical break in any core primitive; key material leak |
| High     | 2 500 – 10 000  | Signature-forgery pathway; envelope-decrypt without recipient key |
| Medium   | 500 – 2 500     | Panic-wipe leaves recoverable traces; timing side-channel with < 2^40 queries |
| Low      | Public credit + a very well-designed thank-you note | Design-doc typo that could mislead an implementer |

Paid in EUR to a bank account or ETH address of the researcher's choice.

## Timeline

- **T+0 to T+3 working days:** we acknowledge the report + open a private triage channel.
- **T+7:** we agree on severity + coordinated-disclosure window (default 90 days).
- **Before disclosure:** we ship the fix across every VF product that depends on the kit, coordinate release notes, and credit the reporter (unless anonymous requested).
- **On disclosure:** advisory published at github.com/vfempire-hq/vf-privacy-kit/security/advisories.

## Legal safe harbour

Good-faith security research on `vf-privacy-kit` is welcomed. We do not pursue legal action against researchers who:

- Test only on shipping releases or their own local builds (not against other users' running instances).
- Don't exfiltrate or damage user data.
- Give us the coordinated-disclosure window before public release.

## Supported versions

The current major version is always supported. When a new major ships, the previous major receives security-only patches for 12 months.

| Version | Supported | Until |
|---------|-----------|-------|
| 0.x     | ✅ (pre-1.0 rolling) | superseded by 1.0 |
| 1.x     | tracked when it ships | tracked when it ships |

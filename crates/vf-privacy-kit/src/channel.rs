// channel.rs — VF Sealed Channel client (Tauri-free port).
//
// The channel is a poll-based transport that hides envelope metadata:
// the server sees only `{ recipient_id: hex(sha256(signing_pub)), sealed_blob }`.
// It does not know sender identity, recipient email, or subject.
//
// This is the reference implementation for `@vf/privacy-kit`. It is HTTP-
// agnostic — pass in your own reqwest::Client. VF Mail's Tauri wrapper
// creates the client and passes it in; other consumers can do the same.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey};
use sha2::{Digest, Sha256};

use crate::identity::Identity;
use crate::mail_crypto;

pub const DEFAULT_CHANNEL_BASE: &str = "https://inbox.vfempire.com";
const CHALLENGE_PREFIX: &[u8] = b"vfch-v1|";

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Compute the recipient_id (hex sha256 of an Ed25519 signing pubkey).
/// The recipient_id is what the server routes by — it does NOT know
/// which email address owns the queue.
pub fn recipient_id_from_signing_pub_b64(signing_pub_b64: &str) -> Result<String> {
    let pub_bytes = B64
        .decode(signing_pub_b64)
        .map_err(|e| anyhow!("bad signing_pub_b64: {e}"))?;
    let mut h = Sha256::new();
    h.update(&pub_bytes);
    Ok(hex(&h.finalize()))
}

fn sign_challenge(id: &Identity, recipient_id: &str, ts: u64) -> Vec<u8> {
    let mut msg = Vec::with_capacity(CHALLENGE_PREFIX.len() + 8 + recipient_id.len());
    msg.extend_from_slice(CHALLENGE_PREFIX);
    msg.extend_from_slice(&ts.to_be_bytes());
    msg.extend_from_slice(recipient_id.as_bytes());
    let sk = SigningKey::from_bytes(&id.signing_seed);
    let sig: Signature = sk.sign(&msg);
    sig.to_bytes().to_vec()
}

fn now_ts() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// A single pulled envelope after decryption + signature verification.
#[derive(Debug, serde::Serialize)]
pub struct PulledMessage {
    pub msg_id: String,
    pub plaintext: String,
    pub sender: String,
    pub signature_verified: bool,
}

/// Send a sealed blob to the recipient's channel queue.
///
/// Anyone can drop a blob into any queue; no server-side auth on send.
/// The recipient's private key is the only thing that can decrypt.
pub async fn send(
    client: &reqwest::Client,
    base_url: &str,
    peer_signing_pub_b64: &str,
    sealed_blob_b64: &str,
) -> Result<String> {
    let recipient_id = recipient_id_from_signing_pub_b64(peer_signing_pub_b64)?;
    let resp = client
        .post(format!("{}/channel/send", base_url))
        .json(&serde_json::json!({
            "recipient_id": recipient_id,
            "sealed_blob_b64": sealed_blob_b64,
        }))
        .send()
        .await?;
    let status = resp.status();
    let body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!("channel/send failed {}: {}", status, body));
    }
    let parsed: serde_json::Value = serde_json::from_str(&body)?;
    Ok(parsed
        .get("msg_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string())
}

/// Pull queued envelopes for this identity's recipient_id.
///
/// Proves ownership of the queue via Ed25519 signed challenge. Server
/// verifies `sha256(signing_pub) == recipient_id` before returning.
/// Each returned blob is decrypted + signature-verified locally.
pub async fn pull(
    client: &reqwest::Client,
    base_url: &str,
    identity: &Identity,
) -> Result<Vec<PulledMessage>> {
    let recipient_id = recipient_id_from_signing_pub_b64(&identity.public.signing_pub_b64)?;
    let ts = now_ts();
    let sig = sign_challenge(identity, &recipient_id, ts);
    let resp = client
        .post(format!("{}/channel/pull", base_url))
        .json(&serde_json::json!({
            "recipient_id": recipient_id,
            "timestamp": ts,
            "signing_pub_b64": identity.public.signing_pub_b64,
            "signature_b64": B64.encode(&sig),
        }))
        .send()
        .await?;
    let status = resp.status();
    let body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!("channel/pull failed {}: {}", status, body));
    }
    let raw: serde_json::Value = serde_json::from_str(&body)?;
    let mut out = Vec::new();
    if let Some(msgs) = raw.get("messages").and_then(|v| v.as_array()) {
        for m in msgs {
            let msg_id = m
                .get("msg_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let sealed_b64 = m.get("sealed_blob_b64").and_then(|v| v.as_str()).unwrap_or("");
            let Ok(sealed_bytes) = B64.decode(sealed_b64) else { continue };
            let mime_body = String::from_utf8_lossy(&sealed_bytes).to_string();
            let opened = mail_crypto::from_mime_body(&mime_body)
                .and_then(|p| mail_crypto::open(identity, &p));
            if let Ok(opened) = opened {
                out.push(PulledMessage {
                    msg_id,
                    plaintext: String::from_utf8_lossy(&opened.plaintext).to_string(),
                    sender: opened.sender,
                    signature_verified: opened.signature_verified,
                });
            }
        }
    }
    Ok(out)
}

/// Acknowledge messages (delete from server queue).
pub async fn ack(
    client: &reqwest::Client,
    base_url: &str,
    identity: &Identity,
    msg_ids: &[String],
) -> Result<usize> {
    if msg_ids.is_empty() {
        return Ok(0);
    }
    let recipient_id = recipient_id_from_signing_pub_b64(&identity.public.signing_pub_b64)?;
    let ts = now_ts();
    let sig = sign_challenge(identity, &recipient_id, ts);
    let resp = client
        .post(format!("{}/channel/ack", base_url))
        .json(&serde_json::json!({
            "recipient_id": recipient_id,
            "timestamp": ts,
            "signing_pub_b64": identity.public.signing_pub_b64,
            "signature_b64": B64.encode(&sig),
            "msg_ids": msg_ids,
        }))
        .send()
        .await?;
    let status = resp.status();
    let body = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!("channel/ack failed {}: {}", status, body));
    }
    let parsed: serde_json::Value = serde_json::from_str(&body)?;
    Ok(parsed
        .get("deleted")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize)
}

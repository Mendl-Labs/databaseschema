//! AES-256-GCM Encryption for Credentials at Rest
//!
//! Replaces the base64 placeholder (`enc:` prefix) with real authenticated
//! encryption. Used by all engines that touch `exchange_credentials`,
//! `webhook_endpoints.secret`, and `domain_ssl_certificates`.
//!
//! ## Key Management
//!
//! - **Development**: `ENCRYPTION_MASTER_KEY` env var (hex-encoded 32 bytes)
//! - **Production**: Azure Key Vault — key wrapping / unwrapping via REST API
//!
//! ## Wire Format
//! Encrypted values are stored as: `aes256gcm:<base64(nonce|ciphertext|tag)>`
//!
//! ## Usage
//! ```ignore
//! use databaseschema::encryption::{encrypt, decrypt, EncryptionKey};
//!
//! let key = EncryptionKey::from_env()?;
//! let encrypted = encrypt("my-secret-api-key", &key)?;
//! let decrypted = decrypt(&encrypted, &key)?;
//! assert_eq!(decrypted, "my-secret-api-key");
//! ```

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, AeadCore, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Prefix for AES-256-GCM encrypted values (distinguishes from legacy `enc:` base64)
const ENCRYPTED_PREFIX: &str = "aes256gcm:";

/// Legacy base64 prefix used by the placeholder implementation
const LEGACY_PREFIX: &str = "enc:";

/// Encryption errors
#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Invalid ciphertext format")]
    InvalidFormat,

    #[error("Missing ENCRYPTION_MASTER_KEY environment variable")]
    MissingKey,

    #[error("Azure Key Vault error: {0}")]
    KeyVaultError(String),

    #[error("Legacy enc: format detected — call decrypt_legacy() or migrate()")]
    LegacyFormat,
}

/// 256-bit encryption key with secure memory clearing on drop
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct EncryptionKey {
    bytes: [u8; 32],
}

impl EncryptionKey {
    /// Create from raw 32-byte key material.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    /// Load from `ENCRYPTION_MASTER_KEY` environment variable (hex-encoded).
    pub fn from_env() -> Result<Self, EncryptionError> {
        let hex_key = std::env::var("ENCRYPTION_MASTER_KEY")
            .map_err(|_| EncryptionError::MissingKey)?;
        Self::from_hex(&hex_key)
    }

    /// Parse from hex-encoded string (64 hex chars = 32 bytes).
    pub fn from_hex(hex: &str) -> Result<Self, EncryptionError> {
        let hex = hex.trim();
        if hex.len() != 64 {
            return Err(EncryptionError::InvalidKey(format!(
                "Expected 64 hex chars (32 bytes), got {}",
                hex.len()
            )));
        }

        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
                .map_err(|e| EncryptionError::InvalidKey(format!("Invalid hex at byte {}: {}", i, e)))?;
        }

        Ok(Self { bytes })
    }

    /// Generate a new random encryption key.
    pub fn generate() -> Self {
        use aes_gcm::aead::rand_core::RngCore;
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self { bytes }
    }

    /// Export as hex string (for initial setup / key rotation).
    pub fn to_hex(&self) -> String {
        self.bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Get the raw key bytes (for AES cipher initialization).
    fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl std::fmt::Debug for EncryptionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EncryptionKey(***)")
    }
}

/// Encrypt a plaintext string with AES-256-GCM.
///
/// Returns: `aes256gcm:<base64(nonce || ciphertext)>`
///
/// The nonce (12 bytes) is randomly generated and prepended to the ciphertext.
/// AES-GCM provides both confidentiality and authentication (AEAD).
pub fn encrypt(plaintext: &str, key: &EncryptionKey) -> Result<String, EncryptionError> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    // Combine nonce + ciphertext for storage
    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);

    Ok(format!("{}{}", ENCRYPTED_PREFIX, STANDARD.encode(&combined)))
}

/// Decrypt an AES-256-GCM encrypted string.
///
/// Accepts: `aes256gcm:<base64(nonce || ciphertext)>`
pub fn decrypt(encrypted: &str, key: &EncryptionKey) -> Result<String, EncryptionError> {
    // Check for legacy format
    if encrypted.starts_with(LEGACY_PREFIX) {
        return Err(EncryptionError::LegacyFormat);
    }

    let encoded = encrypted
        .strip_prefix(ENCRYPTED_PREFIX)
        .ok_or(EncryptionError::InvalidFormat)?;

    let combined = STANDARD
        .decode(encoded)
        .map_err(|_| EncryptionError::InvalidFormat)?;

    if combined.len() < 12 {
        return Err(EncryptionError::InvalidFormat);
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| EncryptionError::DecryptionFailed("Authentication failed".to_string()))?;

    String::from_utf8(plaintext)
        .map_err(|e| EncryptionError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))
}

/// Decrypt a legacy `enc:<base64>` value (for migration compatibility).
pub fn decrypt_legacy(encrypted: &str) -> Result<String, EncryptionError> {
    let encoded = encrypted
        .strip_prefix(LEGACY_PREFIX)
        .ok_or(EncryptionError::InvalidFormat)?;

    let bytes = STANDARD
        .decode(encoded)
        .map_err(|_| EncryptionError::InvalidFormat)?;

    String::from_utf8(bytes)
        .map_err(|e| EncryptionError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))
}

/// Migrate a legacy `enc:<base64>` value to AES-256-GCM encryption.
///
/// Returns the re-encrypted value, or passes through already-encrypted values.
pub fn migrate_legacy(value: &str, key: &EncryptionKey) -> Result<String, EncryptionError> {
    if value.starts_with(ENCRYPTED_PREFIX) {
        // Already using AES-256-GCM
        return Ok(value.to_string());
    }

    if value.starts_with(LEGACY_PREFIX) {
        let plaintext = decrypt_legacy(value)?;
        return encrypt(&plaintext, key);
    }

    // Not encrypted at all — encrypt it
    encrypt(value, key)
}

/// Check if a value is encrypted (either legacy or AES-256-GCM).
pub fn is_encrypted(value: &str) -> bool {
    value.starts_with(ENCRYPTED_PREFIX) || value.starts_with(LEGACY_PREFIX)
}

/// Check if a value uses the current AES-256-GCM format.
pub fn is_current_format(value: &str) -> bool {
    value.starts_with(ENCRYPTED_PREFIX)
}

// ── Azure Key Vault Integration ──────────────────────────────

/// Azure Key Vault key wrapping for production key management.
///
/// Instead of storing `ENCRYPTION_MASTER_KEY` in an env var, the master key
/// is wrapped (encrypted) by Azure Key Vault and stored as a config value.
/// On startup, the wrapped key is unwrapped via the Key Vault REST API.
#[cfg(feature = "azure-keyvault")]
pub mod azure_keyvault {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// Azure Key Vault configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct KeyVaultConfig {
        /// Key Vault URL (e.g., `https://myvault.vault.azure.net`)
        pub vault_url: String,
        /// Key name in Key Vault
        pub key_name: String,
        /// Key version (or empty for latest)
        pub key_version: String,
    }

    impl KeyVaultConfig {
        /// Load from environment variables.
        pub fn from_env() -> Result<Self, EncryptionError> {
            Ok(Self {
                vault_url: std::env::var("AZURE_KEY_VAULT_URL")
                    .map_err(|_| EncryptionError::KeyVaultError("Missing AZURE_KEY_VAULT_URL".into()))?,
                key_name: std::env::var("AZURE_KEY_VAULT_KEY_NAME")
                    .unwrap_or_else(|_| "mendll-encryption-key".to_string()),
                key_version: std::env::var("AZURE_KEY_VAULT_KEY_VERSION")
                    .unwrap_or_default(),
            })
        }
    }

    /// Wrap (encrypt) an encryption key using Azure Key Vault.
    ///
    /// The wrapped key bytes can be safely stored in config/env vars.
    pub async fn wrap_key(
        config: &KeyVaultConfig,
        key: &EncryptionKey,
        access_token: &str,
    ) -> Result<String, EncryptionError> {
        let url = format!(
            "{}/keys/{}/{}/wrapkey?api-version=7.4",
            config.vault_url, config.key_name, config.key_version
        );

        let body = serde_json::json!({
            "alg": "RSA-OAEP-256",
            "value": STANDARD.encode(key.as_bytes()),
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| EncryptionError::KeyVaultError(format!("Request failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(EncryptionError::KeyVaultError(format!(
                "Key Vault wrap failed ({}): {}",
                status, text
            )));
        }

        #[derive(Deserialize)]
        struct WrapResponse {
            value: String,
        }

        let wrap_resp: WrapResponse = resp
            .json()
            .await
            .map_err(|e| EncryptionError::KeyVaultError(format!("Parse failed: {}", e)))?;

        Ok(wrap_resp.value)
    }

    /// Unwrap (decrypt) an encryption key using Azure Key Vault.
    ///
    /// Call this on service startup to recover the master encryption key.
    pub async fn unwrap_key(
        config: &KeyVaultConfig,
        wrapped_key_b64: &str,
        access_token: &str,
    ) -> Result<EncryptionKey, EncryptionError> {
        let url = format!(
            "{}/keys/{}/{}/unwrapkey?api-version=7.4",
            config.vault_url, config.key_name, config.key_version
        );

        let body = serde_json::json!({
            "alg": "RSA-OAEP-256",
            "value": wrapped_key_b64,
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| EncryptionError::KeyVaultError(format!("Request failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(EncryptionError::KeyVaultError(format!(
                "Key Vault unwrap failed ({}): {}",
                status, text
            )));
        }

        #[derive(Deserialize)]
        struct UnwrapResponse {
            value: String,
        }

        let unwrap_resp: UnwrapResponse = resp
            .json()
            .await
            .map_err(|e| EncryptionError::KeyVaultError(format!("Parse failed: {}", e)))?;

        let key_bytes = STANDARD
            .decode(&unwrap_resp.value)
            .map_err(|_| EncryptionError::KeyVaultError("Invalid base64 in unwrap response".into()))?;

        if key_bytes.len() != 32 {
            return Err(EncryptionError::InvalidKey(format!(
                "Unwrapped key is {} bytes, expected 32",
                key_bytes.len()
            )));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&key_bytes);
        Ok(EncryptionKey::from_bytes(bytes))
    }
}

// ── Tests ────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = EncryptionKey::generate();
        let plaintext = "super-secret-api-key-12345";

        let encrypted = encrypt(plaintext, &key).unwrap();
        assert!(encrypted.starts_with(ENCRYPTED_PREFIX));
        assert_ne!(encrypted, plaintext);

        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_produces_different_ciphertexts() {
        let key = EncryptionKey::generate();
        let plaintext = "same-plaintext";

        let enc1 = encrypt(plaintext, &key).unwrap();
        let enc2 = encrypt(plaintext, &key).unwrap();

        // Different nonces → different ciphertexts (semantic security)
        assert_ne!(enc1, enc2);

        // Both decrypt to the same value
        assert_eq!(decrypt(&enc1, &key).unwrap(), plaintext);
        assert_eq!(decrypt(&enc2, &key).unwrap(), plaintext);
    }

    #[test]
    fn test_wrong_key_fails_decryption() {
        let key1 = EncryptionKey::generate();
        let key2 = EncryptionKey::generate();

        let encrypted = encrypt("my-secret", &key1).unwrap();

        let result = decrypt(&encrypted, &key2);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_ciphertext_fails() {
        let key = EncryptionKey::generate();
        let encrypted = encrypt("my-secret", &key).unwrap();

        // Flip a bit in the base64 payload
        let mut tampered = encrypted.clone();
        let last_char = tampered.pop().unwrap();
        let replacement = if last_char == 'A' { 'B' } else { 'A' };
        tampered.push(replacement);

        let result = decrypt(&tampered, &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_legacy_format_detection() {
        let key = EncryptionKey::generate();
        let legacy = "enc:aGVsbG8gd29ybGQ=";

        let result = decrypt(legacy, &key);
        assert!(matches!(result, Err(EncryptionError::LegacyFormat)));
    }

    #[test]
    fn test_decrypt_legacy() {
        let decrypted = decrypt_legacy("enc:aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(decrypted, "hello world");
    }

    #[test]
    fn test_migrate_legacy() {
        let key = EncryptionKey::generate();
        let legacy = "enc:c2VjcmV0";

        let migrated = migrate_legacy(legacy, &key).unwrap();
        assert!(migrated.starts_with(ENCRYPTED_PREFIX));

        let decrypted = decrypt(&migrated, &key).unwrap();
        assert_eq!(decrypted, "secret");
    }

    #[test]
    fn test_migrate_already_encrypted() {
        let key = EncryptionKey::generate();
        let encrypted = encrypt("test", &key).unwrap();

        let migrated = migrate_legacy(&encrypted, &key).unwrap();
        assert_eq!(migrated, encrypted); // No change
    }

    #[test]
    fn test_key_from_hex() {
        let key = EncryptionKey::generate();
        let hex = key.to_hex();

        let restored = EncryptionKey::from_hex(&hex).unwrap();
        assert_eq!(key.as_bytes(), restored.as_bytes());
    }

    #[test]
    fn test_is_encrypted() {
        assert!(is_encrypted("aes256gcm:abc123"));
        assert!(is_encrypted("enc:abc123"));
        assert!(!is_encrypted("plain-text"));
    }

    #[test]
    fn test_empty_plaintext() {
        let key = EncryptionKey::generate();
        let encrypted = encrypt("", &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_unicode_plaintext() {
        let key = EncryptionKey::generate();
        let plaintext = "🔐 密钥 مفتاح κλειδί";
        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}

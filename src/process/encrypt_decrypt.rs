use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, Key, KeyInit, Nonce, OsRng},
    XChaCha20Poly1305,
};

pub struct EncryptResult {
    pub key_base64: String,
    pub nonce_base64: String,
    pub ciphertext_base64: String,
}

pub trait TextEncryptor {
    fn encrypt(data: &str) -> anyhow::Result<EncryptResult>;
}
pub trait TextDecrypor {
    fn decrypt(data: &str, key_base64: &str, nonce_base64: &str) -> anyhow::Result<String>;
}

pub struct ChaCha20Poly1305 {}

impl TextEncryptor for ChaCha20Poly1305 {
    fn encrypt(data: &str) -> anyhow::Result<EncryptResult> {
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        let cipher = XChaCha20Poly1305::new(&key);
        let ciphertext = cipher
            .encrypt(&nonce, data.as_ref())
            .map_err(|e| anyhow::anyhow!(format!("{}", e)))?;

        let key_base64 = BASE64_STANDARD.encode(key);
        let nonce_base64 = BASE64_STANDARD.encode(nonce);
        let ciphertext_base64 = BASE64_STANDARD.encode(ciphertext);

        Ok(EncryptResult {
            key_base64,
            nonce_base64,
            ciphertext_base64,
        })
    }
}

impl TextDecrypor for ChaCha20Poly1305 {
    fn decrypt(data: &str, key_base64: &str, nonce_base64: &str) -> anyhow::Result<String> {
        let ciphertext_bytes = BASE64_STANDARD.decode(data)?;
        let key_bytes = BASE64_STANDARD.decode(key_base64)?;
        let nonce_bytes = BASE64_STANDARD.decode(nonce_base64)?;

        // let mut nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        // nonce.clone_from_slice(&nonce_bytes);

        // let nonce = GenericArray::<u8, <XChaCha20Poly1305 as AeadCore>::NonceSize>::clone_from_slice(
        //     &nonce_bytes,
        // );

        let nonce = Nonce::<XChaCha20Poly1305>::from_slice(&nonce_bytes);
        let key = Key::<XChaCha20Poly1305>::from_slice(&key_bytes);

        let cipher = XChaCha20Poly1305::new(key);
        // let cipher =
        //     XChaCha20Poly1305::new_from_slice(&key_bytes).map_err(|e| anyhow!(format!("{}", e)))?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext_bytes.as_ref())
            .map_err(|e| anyhow::anyhow!(format!("{}", e)))?;
        Ok(String::from_utf8(plaintext)?)
    }
}

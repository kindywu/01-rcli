use anyhow::anyhow;
use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, AeadCore, Key, KeyInit, Nonce, OsRng},
    XChaCha20Poly1305,
};
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;
use ed25519_dalek::SIGNATURE_LENGTH;

use crate::cli::TextSignFormat;

pub struct EncryptResult {
    pub key_base64: String,
    pub nonce_base64: String,
    pub ciphertext_base64: String,
}

pub fn process_text_encrypt(plain_text: &str) -> anyhow::Result<EncryptResult> {
    let key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let ciphertext = cipher
        .encrypt(&nonce, plain_text.as_ref())
        .map_err(|e| anyhow!(format!("{}", e)))?;

    let key_base64 = BASE64_STANDARD.encode(key);
    let nonce_base64 = BASE64_STANDARD.encode(nonce);
    let ciphertext_base64 = BASE64_STANDARD.encode(ciphertext);

    Ok(EncryptResult {
        key_base64,
        nonce_base64,
        ciphertext_base64,
    })
}

pub fn process_text_decrypt(
    cipher_text: &str,
    key_base64: &str,
    nonce_base64: &str,
) -> anyhow::Result<String> {
    let ciphertext_bytes = BASE64_STANDARD.decode(cipher_text)?;
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
        .map_err(|e| anyhow!(format!("{}", e)))?;
    Ok(String::from_utf8(plaintext)?)
}

pub fn process_text_sign(data: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let key = key.as_bytes()[..32].try_into()?;
    let signed = match format {
        TextSignFormat::Blake3 => blake3::keyed_hash(&key, data.as_bytes()).to_string(),
        TextSignFormat::Ed25519 => {
            let signing_key = SigningKey::from_bytes(&key);
            let signature = signing_key.sign(data.as_bytes());
            let signature_bytes: [u8; SIGNATURE_LENGTH] = signature.to_bytes();
            BASE64_STANDARD.encode(signature_bytes)
        }
    };
    Ok(signed)
}

pub fn process_text_verify(
    data: &str,
    key: &str,
    signed: &str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let key = key.as_bytes()[..32].try_into()?;
    let verify = match format {
        TextSignFormat::Blake3 => blake3::keyed_hash(&key, data.as_bytes()).to_string() == signed,
        TextSignFormat::Ed25519 => {
            let signature_bytes = BASE64_STANDARD.decode(signed)?;
            let signature: Signature = Signature::try_from(&signature_bytes[..])?;
            let signing_key = SigningKey::from_bytes(&key);
            let verifying_key: VerifyingKey = signing_key.verifying_key();
            verifying_key.verify(data.as_bytes(), &signature).is_ok()
        }
    };
    Ok(verify)
}

// cargo run gen-pass | Out-File -FilePath "fixtures/blake3.txt" -Encoding UTF8 -NoNewline

// window: make sure your powershell's $PSVersionTable.PSVersion > 7
// cargo run text encrypt --input fixtures/chacha20poly1305_plain.txt | Out-File -FilePath "fixtures/chacha20poly1305_cipher.txt" -Encoding UTF8 -NoNewline
// cargo run text decrypt --input fixtures/chacha20poly1305_cipher.txt -k aRqQuWdfHfKZg0z5c+gxRTzxk96cSDh4dYpVGJt7mxc= -n F8R0XGeCto1RZlMNMYoQ7qfhpdbQD0Qh

// cargo run text sign -i fixtures\blake3_plain.txt -k fixtures\blake3.txt
// cargo run text verify -i fixtures\blake3_plain.txt -k fixtures\blake3.txt

// cargo run text sign -i fixtures\ed25519_plain.txt -k fixtures\ed25519.txt -f ed25519
// cargo run text verify -i fixtures\ed25519_plain.txt -k fixtures\ed25519.txt -f ed25519

#[cfg(test)]
mod tests {

    use crate::{
        process_text_decrypt, process_text_encrypt, process_text_sign, process_text_verify,
    };

    #[test]
    fn test_process_text_encrypt_decrypt() -> anyhow::Result<()> {
        let plain_text = "hello world";
        let result = process_text_encrypt(plain_text)?;
        let decrypt_text = process_text_decrypt(
            &result.ciphertext_base64,
            &result.key_base64,
            &result.nonce_base64,
        )?;
        // println!("{}", result.ciphertext_base64);
        // println!("{}", result.key_base64);
        // println!("{}", result.nonce_base64);
        assert_eq!(plain_text, decrypt_text);
        Ok(())
    }

    #[test]
    fn test_process_text_sign_verify_blake3() -> anyhow::Result<()> {
        let data = "hello world";
        let key = "eyW2pW29DLaVHe8N3@^Ve?*k@sbEgNFq";
        let signed = process_text_sign(data, key, crate::cli::TextSignFormat::Blake3)?;
        // println!("{}", signed);
        let is_ok = process_text_verify(data, key, &signed, crate::cli::TextSignFormat::Blake3)?;
        assert!(is_ok);
        Ok(())
    }

    #[test]
    fn test_process_text_sign_verify_ed25519() -> anyhow::Result<()> {
        let data = "hello world";
        let key = "eyW2pW29DLaVHe8N3@^Ve?*k@sbEgNFq";
        let signed = process_text_sign(data, key, crate::cli::TextSignFormat::Ed25519)?;
        // println!("{}", signed);
        let is_ok = process_text_verify(data, key, &signed, crate::cli::TextSignFormat::Ed25519)?;
        assert!(is_ok);
        Ok(())
    }
}

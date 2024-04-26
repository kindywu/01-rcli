use crate::cli::TextSignFormat;

use super::{
    encrypt_decrypt::{self, *},
    signer_verifier::*,
};

pub fn process_text_encrypt(plain_text: &str) -> anyhow::Result<EncryptResult> {
    encrypt_decrypt::ChaCha20Poly1305::encrypt(plain_text)
}

pub fn process_text_decrypt(
    cipher_text: &str,
    key_base64: &str,
    nonce_base64: &str,
) -> anyhow::Result<String> {
    encrypt_decrypt::ChaCha20Poly1305::decrypt(cipher_text, key_base64, nonce_base64)
}

pub fn process_text_sign(data: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519::try_new(key)?),
    };
    let signed = signer.sign(data)?;
    Ok(signed)
}

pub fn process_text_verify(
    data: &str,
    key: &str,
    signed: &str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519::try_new(key)?),
    };
    let verify = verifier.verify(data, signed)?;
    Ok(verify)
}

pub fn process_text_generate_key(format: TextSignFormat) -> anyhow::Result<Vec<String>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate_key(),
        TextSignFormat::Ed25519 => Ed25519::generate_key(),
    }
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

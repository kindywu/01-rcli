use base64::prelude::*;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;
use ed25519_dalek::SIGNATURE_LENGTH;
use rand::rngs::OsRng;

use super::gen_pass;
pub trait KeyGenerator {
    fn generate_key() -> anyhow::Result<Vec<String>>;
}

pub trait TextSigner {
    fn sign(&self, data: &str) -> anyhow::Result<String>;
}
pub trait TextVerifier {
    fn verify(&self, data: &str, signed: &str) -> anyhow::Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, data: &str) -> anyhow::Result<String> {
        Ok(blake3::keyed_hash(&self.key, data.as_bytes()).to_string())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, data: &str, signed: &str) -> anyhow::Result<bool> {
        Ok(blake3::keyed_hash(&self.key, data.as_bytes()).to_string() == signed)
    }
}

impl KeyGenerator for Blake3 {
    fn generate_key() -> anyhow::Result<Vec<String>> {
        let key = gen_pass::process_gen_pass(32, false, false, false, false)?;
        let keys = vec![key];
        Ok(keys)
    }
}

pub struct Ed25519 {
    key: SigningKey,
}

impl Ed25519 {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(&key);
        Self { key: signing_key }
    }
}

impl TextSigner for Ed25519 {
    fn sign(&self, data: &str) -> anyhow::Result<String> {
        let signature = &self.key.sign(data.as_bytes());
        let signature_bytes: [u8; SIGNATURE_LENGTH] = signature.to_bytes();
        Ok(BASE64_STANDARD.encode(signature_bytes))
    }
}

impl TextVerifier for Ed25519 {
    fn verify(&self, data: &str, signed: &str) -> anyhow::Result<bool> {
        let signature_bytes = BASE64_STANDARD.decode(signed)?;
        let signature: Signature = Signature::try_from(&signature_bytes[..])?;
        let verifying_key: VerifyingKey = self.key.verifying_key();
        Ok(verifying_key.verify(data.as_bytes(), &signature).is_ok())
    }
}

impl KeyGenerator for Ed25519 {
    fn generate_key() -> anyhow::Result<Vec<String>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let verifying_key: VerifyingKey = signing_key.verifying_key();
        let keys = vec![
            BASE64_STANDARD.encode(signing_key.to_bytes()),
            BASE64_STANDARD.encode(verifying_key.to_bytes()),
        ];
        Ok(keys)
    }
}

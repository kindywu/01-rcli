use std::{fmt, path::PathBuf, str::FromStr};

use clap::{command, Parser};
use enum_dispatch::enum_dispatch;

use crate::{
    process_text_decrypt, process_text_encrypt, process_text_generate_key, process_text_sign,
    process_text_verify, read_content, CmdExector,
};

use super::{verify_file, verify_path};
#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(about = "encode input to base64")]
    Encrypt(EncryptOpts),
    #[command(about = "decode input from base64")]
    Decrypt(DecryptOpts),

    #[command(about = "sign a message with a private/public key")]
    Sign(SignOpts),
    #[command(about = "verify a signed message")]
    Verify(VerifytOpts),
    #[command(about = "Generate a new key pair")]
    GenerateKey(GenerateKeytOpts),
}
#[derive(Debug, Parser)]

pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Parser)]

pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long)]
    pub key_base64: String,

    #[arg(short, long)]
    pub nonce_base64: String,
}

#[derive(Clone, Copy, Debug)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,

    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifytOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub signed: String,

    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,

    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateKeytOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short,long,value_parser=verify_path, default_value="fixtures")]
    pub path: PathBuf,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

// let name: &str = OutputFormat::Json.into();
impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("encrypt {:?}", self);
        let plain_text = read_content(&self.input)?;
        let encrypt_result = process_text_encrypt(plain_text.trim())?;
        println!("{}", encrypt_result.ciphertext_base64);

        eprintln!("encrypt text is {}", encrypt_result.ciphertext_base64);
        eprintln!("Make sure to save the values of the key and nonce for decryption later!");
        eprintln!(
            "key is {} nonce is {}",
            encrypt_result.key_base64, encrypt_result.nonce_base64
        );
        Ok(())
    }
}
impl CmdExector for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("decrypt {:?}", self);
        let cipher_text = read_content(&self.input)?;
        let plain_text =
            process_text_decrypt(cipher_text.trim(), &self.key_base64, &self.nonce_base64)?;
        println!("plain_text is {}", plain_text);
        Ok(())
    }
}
impl CmdExector for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("sign {:?}", self);
        let data = read_content(&self.input)?;
        let key = read_content(&self.key)?;

        // println!("plain_text is {}", plain_text);
        let signed = process_text_sign(data.as_str(), key.as_str(), self.format)?;
        println!("{}", signed);
        Ok(())
    }
}
impl CmdExector for VerifytOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("verify {:?}", self);
        let data = read_content(&self.input)?;
        let key = read_content(&self.key)?;
        let signed = read_content(&self.signed)?;
        eprintln!("verify {:?}", self);
        let verify =
            process_text_verify(data.as_str(), key.as_str(), signed.as_str(), self.format)?;
        println!("{}", verify);
        Ok(())
    }
}
impl CmdExector for GenerateKeytOpts {
    async fn execute(self) -> anyhow::Result<()> {
        // eprintln!("generate key {:?}", opts);
        let keys = process_text_generate_key(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let content = keys
                    .first()
                    .ok_or(anyhow::anyhow!("key is empty"))?
                    .as_bytes();
                println!("{:?}", content);
                std::fs::write(self.path.join("blake.txt"), content)?
            }
            TextSignFormat::Ed25519 => {
                let content = keys
                    .first()
                    .ok_or(anyhow::anyhow!("secret key is empty"))?
                    .as_bytes();
                println!("{:?}", content);

                std::fs::write(self.path.join("ed25519.sk"), content)?;
                let content = keys
                    .get(1)
                    .ok_or(anyhow::anyhow!("public key is empty"))?
                    .as_bytes();
                println!("{:?}", content);

                std::fs::write(self.path.join("ed25519.pk"), content)?
            }
        };
        Ok(())
    }
}

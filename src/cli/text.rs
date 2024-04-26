use std::{fmt, str::FromStr};

use clap::{command, Parser};

use super::verify_file;
#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "encode input to base64")]
    Encrypt(EncryptOpts),
    #[command(about = "decode input from base64")]
    Decrypt(DecryptOpts),

    #[command(about = "sign a message with a private/public key")]
    Sign(SignOpts),
    #[command(about = "verify a signed message")]
    Verify(VerifytOpts),
    #[command(about = "Generate a key pair")]
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
pub struct GenerateKeytOpts {}

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

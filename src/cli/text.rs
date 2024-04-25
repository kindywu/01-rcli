use clap::{command, Parser};

use super::verify_file;
#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "encode input to base64")]
    Encrypt(EncryptOpts),
    #[command(about = "decode input from base64")]
    Decrypt(DecryptOpts),
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

use clap::Parser;
use jsonwebtoken::Algorithm;

use super::verify_file;

#[derive(Debug, Parser)]

pub enum JwtSubCommand {
    #[command(about = "sign a message with a private/public key")]
    Sign(SignOpts),
    #[command(about = "verify a signed message")]
    Verify(VerifytOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, default_value = "HS256")]
    pub algorithm: Algorithm,

    #[arg(short, long, default_value = "secret")]
    pub key: String,

    #[arg(short, long)]
    pub sub: String,

    #[arg(long)]
    pub aud: String,

    #[arg(short, long, default_value_t = 3)]
    pub exp: u64,

    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct VerifytOpts {
    #[arg(short, long, default_value = "HS256")]
    pub algorithm: Algorithm,

    #[arg(short, long, default_value = "secret")]
    pub key: String,

    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

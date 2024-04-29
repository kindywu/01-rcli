use chrono::Local;
use clap::Parser;
use duration_str::parse;
use enum_dispatch::enum_dispatch;
use jsonwebtoken::Algorithm;
use std::ops::Add;

use crate::{process_jwt_sign, process_jwt_verify, read_content, CmdExector};

use super::verify_file;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "sign a message with a private/public key")]
    Sign(JwtSignOpts),
    #[command(about = "verify a signed message")]
    Verify(JwtVerifytOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long, default_value = "HS256")]
    pub algorithm: Algorithm,

    #[arg(short, long, default_value = "secret")]
    pub key: String,

    #[arg(short, long)]
    pub sub: String,

    #[arg(long)]
    pub aud: String,

    #[arg(short, long, default_value = "3s")]
    pub exp: String,

    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifytOpts {
    #[arg(short, long, default_value = "HS256")]
    pub algorithm: Algorithm,

    #[arg(short, long, default_value = "secret")]
    pub key: String,

    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("sign {:?}", self);
        let data = read_content(&self.input)?;
        // println!("plain_text is {}", data);
        let exp = expiration_timestamp(self.exp) as u64;
        let signed = process_jwt_sign(self.algorithm, self.key, self.aud, self.sub, exp, data)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExector for JwtVerifytOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("verify {:?}", self);
        let signed = read_content(&self.input)?;
        let claims = process_jwt_verify(self.algorithm, self.key, signed);
        println!("{:?}", claims);
        Ok(())
    }
}

fn expiration_timestamp(exp: String) -> i64 {
    let duration = parse(exp).unwrap();
    let now = Local::now();
    let tomorrow = now.add(duration);
    tomorrow.timestamp()
}

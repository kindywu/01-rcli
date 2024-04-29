use std::fmt;
use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_base64_decode, process_base64_encode, read_content, CmdExector};

use super::verify_file;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(about = "encode input to base64")]
    Encode(EncodeOpts),
    #[command(about = "decode input from base64")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

// let name: &str = OutputFormat::Json.into();
impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("encode {:?}", self);
        let data = read_content(&self.input)?;
        let encode = process_base64_encode(data.as_str(), self.format)?;
        println!("{}", encode);
        Ok(())
    }
}

impl CmdExector for DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        eprintln!("decode {:?}", self);
        let data = read_content(&self.input)?;
        let decode = process_base64_decode(&data, self.format)?;
        println!("{}", decode);
        Ok(())
    }
}

// impl CmdExector for Base64SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             Base64SubCommand::Encode(opts) => opts.execute().await,
//             Base64SubCommand::Decode(opts) => opts.execute().await,
//         }
//     }
// }

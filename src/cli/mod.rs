mod base64;
mod cvs;
mod gen_pass;
mod http;
mod jwt;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::{Path, PathBuf};

pub use base64::*;
pub use cvs::*;
pub use gen_pass::*;
pub use http::*;
pub use jwt::*;
pub use text::*;

// use crate::CmdExector;

#[derive(Debug, Parser)]
#[command(name ="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(about = "Show CSV ,or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode and Decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Encrypt and Decrypt text")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Sign and Verify text use JWT")]
    Jwt(JwtSubCommand),
    #[command(subcommand, about = "start http server to serve static file")]
    Http(HttpSubCommand),
}

// &'static 静态->Data段
fn verify_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path_name: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path_name);
    if path.exists() && path.is_dir() {
        Ok(path_name.into())
    } else {
        Err("Path does not exist")
    }
}

// impl CmdExector for SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::Csv(opts) => opts.execute().await,
//             SubCommand::GenPass(opts) => opts.execute().await,
//             SubCommand::Base64(sub) => sub.execute().await,
//             SubCommand::Text(sub) => sub.execute().await,
//             SubCommand::Jwt(sub) => sub.execute().await,
//             SubCommand::Http(sub) => sub.execute().await,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::cli::verify_file;
    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}

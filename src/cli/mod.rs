mod base64;
mod cvs;
mod gen_pass;
mod text;

use clap::Parser;
use std::path::Path;

pub use base64::*;
pub use cvs::*;
pub use gen_pass::*;
pub use text::*;

#[derive(Debug, Parser)]
#[command(name ="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(about = "Show CSV ,or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(about = "Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode and Decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Encrypt and Decrypt text")]
    Text(TextSubCommand),
}

// &'static 静态->Data段
fn verify_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

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

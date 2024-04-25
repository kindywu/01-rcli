mod base64;
mod cvs;
mod gen_pass;

use std::path::Path;

pub use base64::*;
use clap::Parser;
pub use cvs::*;
pub use gen_pass::*;

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
}

// &'static 静态->Data段
fn verify_file(file_name: &str) -> Result<String, &'static str> {
    assert!(!file_name.is_empty(), "file name can't be empty");
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

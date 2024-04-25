// mod 引用
mod base64;
mod csv;
mod gen_pass;
mod text;

use std::{
    fs::File,
    io::{stdin, Read},
};

// pub use 导出
pub use base64::*;
pub use csv::process_csv;
pub use gen_pass::process_gen_pass;
pub use text::*;

// windows: use ctrl+z to finish stdin input
pub fn read_content(input: &str) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).trim().to_owned())
}

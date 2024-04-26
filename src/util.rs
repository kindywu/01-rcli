use std::{
    fs::File,
    io::{stdin, Read},
};

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

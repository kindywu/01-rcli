use clap::Parser;

use std::{fmt, path::Path};

// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Parser)]
#[command(name ="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV ,or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
    Proto,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Toml => write!(f, "toml"),
            OutputFormat::Proto => write!(f, "proto"),
        }
    }
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file_is_exist)]
    pub input: String,
    #[arg(short, long)] //"output.json".into()
    pub output: Option<String>,
    #[arg(short, long,  value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// &'static 静态->Data段
fn verify_file_is_exist(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
    match format.to_lowercase().as_str() {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        "toml" => Ok(OutputFormat::Toml),
        _ => Err("Unsupported format"),
    }
}

use clap::Parser;

use std::{fmt, str::FromStr};

use super::verify_file;

// rcli csv -i input.csv -o output.json --header -d ','

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
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

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
    Proto,
}

// fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
//     match format.to_lowercase().as_str() {
//         "json" => Ok(OutputFormat::Json),
//         "yaml" => Ok(OutputFormat::Yaml),
//         "toml" => Ok(OutputFormat::Toml),
//         _ => Err("Unsupported format"),
//     }
// }

fn parse_format(format: &str) -> Result<OutputFormat, &'static str> {
    format.parse()
}

// impl fmt::Display for OutputFormat {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             OutputFormat::Json => write!(f, "json"),
//             OutputFormat::Yaml => write!(f, "yaml"),
//             OutputFormat::Toml => write!(f, "toml"),
//             OutputFormat::Proto => write!(f, "proto"),
//         }
//     }
// }

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

// let name: &str = OutputFormat::Json.into();
impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
            OutputFormat::Proto => "proto",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            "proto" => Ok(OutputFormat::Proto),
            _ => Err("Invalid format"),
        }
    }
}

// impl TryFrom<&str> for OutputFormat {
//     type Error = &'static str;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "json" => Ok(OutputFormat::Json),
//             "yaml" => Ok(OutputFormat::Yaml),
//             "toml" => Ok(OutputFormat::Toml),
//             "proto" => Ok(OutputFormat::Proto),
//             v => Err("Unsupported format"),
//         }
//     }
// }

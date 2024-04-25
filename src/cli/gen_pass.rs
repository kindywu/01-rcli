use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = false)]
    pub no_upper_case: bool,
    #[arg(long, default_value_t = false)]
    pub no_lower_case: bool,
    #[arg(long, default_value_t = false)]
    pub no_number: bool,
    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

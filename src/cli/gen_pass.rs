use clap::Parser;

use crate::{process_gen_pass, CmdExector};
use zxcvbn::zxcvbn;

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

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_gen_pass(
            self.length,
            self.no_upper_case,
            self.no_lower_case,
            self.no_number,
            self.no_symbol,
        )?;
        println!("{}", password);
        let estimate = zxcvbn(&password, &[]).unwrap();
        eprintln!("password strength score is {}", estimate.score());
        Ok(())
    }
}

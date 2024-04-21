use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};
use std::fs;

// cargo run -- csv -i assets/juventus.csv
// select * from read_csv('assets/juventus.csv', auto_detect=true);
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    // println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let json = process_csv(&opts.input)?;
            fs::write(&opts.output, json)?;
        }
    }
    Ok(())
}

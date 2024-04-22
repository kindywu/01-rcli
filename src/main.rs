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
            let json = process_csv(&opts.input, &opts.format)?;
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", &opts.format)
            };
            // println!("{}", output);
            fs::write(output, json)?;
        }
    }
    Ok(())
}

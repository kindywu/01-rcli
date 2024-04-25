use clap::Parser;
use rcli::{process_csv, process_gen_pass, Opts, SubCommand};
use std::fs;
use zxcvbn::zxcvbn;

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
        SubCommand::GenPass(opts) => {
            let password = process_gen_pass(
                opts.length,
                opts.no_upper_case,
                opts.no_lower_case,
                opts.no_number,
                opts.no_symbol,
            )?;
            println!("{}", password);
            let estimate = zxcvbn(&password, &[]).unwrap();
            eprintln!("password strength score is {}", estimate.score());
        }
    }
    Ok(())
}

mod cli;
mod process;

pub use cli::{Base64SubCommand, Opts, SubCommand};
pub use process::process_csv;
pub use process::process_gen_pass;

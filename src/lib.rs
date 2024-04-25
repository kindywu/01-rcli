mod cli;
mod process;

pub use cli::{Base64SubCommand, Opts, SubCommand, TextSubCommand};
pub use process::process_csv;
pub use process::process_gen_pass;
pub use process::read_content;
pub use process::{process_base64_decode, process_base64_encode};
pub use process::{process_text_decrypt, process_text_encrypt};

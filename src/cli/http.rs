use std::path::PathBuf;

use clap::{command, Parser};

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "serve as a static http server")]
    Serve(ServeOpts),
}

#[derive(Debug, Parser)]
pub struct ServeOpts {
    #[arg(short,long,value_parser=verify_path, default_value="fixtures")]
    pub path: PathBuf,

    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

use std::path::PathBuf;

use clap::{command, Parser};
// use enum_dispatch::enum_dispatch;

use crate::{process_http_serve, CmdExector};

use super::verify_path;

#[derive(Debug, Parser)]
// #[enum_dispatch(CmdExector)]
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

impl CmdExector for ServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.path, self.port).await
    }
}

impl CmdExector for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(opts) => opts.execute().await,
        }
    }
}

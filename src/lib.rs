mod cli;
mod process;
mod util;

use enum_dispatch::enum_dispatch;

pub use cli::*;
pub use process::*;
pub use util::read_content;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}

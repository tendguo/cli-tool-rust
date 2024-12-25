mod cli;
mod process;

pub use cli::*;
pub use process::*;

use enum_dispatch::enum_dispatch;

#[trait_variant::make(HttpService: Send)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn executor(self) -> anyhow::Result<()>;
}

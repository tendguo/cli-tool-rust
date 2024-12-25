mod cli;
mod process;

pub use cli::*;
pub use process::*;

use enum_dispatch::enum_dispatch;

/// https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
#[trait_variant::make(HttpService: Send)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn executor(self) -> anyhow::Result<()>;
}

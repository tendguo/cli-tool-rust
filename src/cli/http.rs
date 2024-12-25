use std::path::PathBuf;

use crate::CmdExecutor;

use super::verify_input_path;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum ServeSubCommand {
    #[command(name = "serve", about = "build a file server")]
    Serve(ServeOpts),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ServeOpts {
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    #[arg(short,long, value_parser = verify_input_path, default_value = ".")]
    pub dir: PathBuf,
}

impl CmdExecutor for ServeOpts {
    async fn executor(self) -> anyhow::Result<()> {
        crate::process_http_serve(self.dir, self.port).await
    }
}

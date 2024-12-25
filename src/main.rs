use clap::Parser;
use handle_csv::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let args: Opts = Opts::parse();
    args.cmd.executor().await?;
    Ok(())
}

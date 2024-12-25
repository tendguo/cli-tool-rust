use anyhow::Ok;
use clap::Parser;

use crate::CmdExecutor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GenArgs {
    /// Name of the person to greet
    #[arg(long, default_value_t = 16)]
    pub length: u8,

    /// Number of times to greet
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl CmdExecutor for GenArgs {
    async fn executor(self) -> anyhow::Result<()> {
        let _ = crate::process_genpassword(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
        );
        Ok(())
    }
}

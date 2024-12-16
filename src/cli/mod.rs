use std::path::Path;

mod base64;
mod csv;
mod genpass;
pub use base64::{Base64Format, Base64Subcommand};
pub use csv::CsvArgs;
pub use genpass::{GenArgs, OutputFormat};

use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvArgs),
    #[command(name = "gen_pass", about = "generate a password")]
    GenPass(GenArgs),
    #[command(subcommand, name = "base64", about = "process base64 encode/decode")]
    Base64(Base64Subcommand),
}

pub fn verify_input_text(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("THE FILE NAME IS NOT EXISTS")
    }
}

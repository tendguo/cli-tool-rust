use std::{path::PathBuf, str::FromStr};

use anyhow::Ok;
use clap::Parser;

use super::{verify_input_path, verify_input_text};

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(name = "generate", about = "generate a key")]
    Generate(GenerateOpts),
    #[command(name = "sign", about = "sign a message through the key")]
    Sign(SignerOpts),
    #[command(name = "verify", about = "verify the signed message")]
    Verify(VerifierOpts),
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GenerateOpts {
    #[arg(short, long, value_parser = verify_input_path)]
    pub path: PathBuf,

    #[arg(short,long, value_parser = parse_format_text, default_value = "ed25519")]
    pub format: TextSignFormat,
}

#[derive(Clone, Debug)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SignerOpts {
    #[arg(short, long, value_parser = verify_input_text, default_value = "-")]
    pub message: String,
    #[arg(short,long, value_parser = parse_format_text, default_value = "ed25519")]
    pub format: TextSignFormat,
    #[arg(short,long, value_parser = verify_input_text, default_value = "-")]
    pub key: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct VerifierOpts {
    #[arg(short, long, value_parser = verify_input_text, default_value = "-")]
    pub message: String,
    #[arg(short,long, value_parser = parse_format_text, default_value = "ed25519")]
    pub format: TextSignFormat,
    #[arg(short,long, value_parser = verify_input_text, default_value = "-")]
    pub key: String,
    #[arg(long)]
    pub sig: String,
}

fn parse_format_text(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

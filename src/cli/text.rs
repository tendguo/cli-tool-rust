use std::{fs::File, io::Write, path::PathBuf, str::FromStr};

use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

use super::{verify_input_path, verify_input_text};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
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

impl CmdExecutor for GenerateOpts {
    async fn executor(self) -> anyhow::Result<()> {
        let key_map = crate::process_generate_key(self.format)?;
        for (k, v) in key_map {
            let mut file = File::create(self.path.join(k)).unwrap();
            file.write_all(&v).unwrap();
        }
        Ok(())
    }
}

impl CmdExecutor for SignerOpts {
    async fn executor(self) -> anyhow::Result<()> {
        let key = crate::get_content(&self.key)?;
        let message = crate::get_content(&self.message)?;
        crate::process_sign_text(self.format, &key, &message)
    }
}

impl CmdExecutor for VerifierOpts {
    async fn executor(self) -> anyhow::Result<()> {
        let key = crate::get_content(&self.key)?;
        let message = crate::get_content(&self.message)?;
        let result = crate::process_sign_verify(self.format, &key, &self.sig, &message)?;
        if result {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}

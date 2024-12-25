use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

use crate::cli::verify_input_text;
use crate::CmdExecutor;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Show CSV, or convert CSV to other formats")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "generate a password")]
    Decode(Base64DecodeOpts),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_text, default_value = "-")]
    pub input: String,

    #[arg(short,long, value_parser = parse_format_base64, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_text, default_value = "-")]
    pub input: String,

    #[arg(short,long, value_parser = parse_format_base64, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Clone, Debug)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_format_base64(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl CmdExecutor for Base64EncodeOpts {
    async fn executor(self) -> anyhow::Result<()> {
        crate::encode_base64(&self.input, self.format)
    }
}

impl CmdExecutor for Base64DecodeOpts {
    async fn executor(self) -> anyhow::Result<()> {
        crate::decode_base64(&self.input, self.format)
    }
}

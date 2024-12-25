use crate::{cli::OutputFormat, CmdExecutor};
use clap::Parser;
use std::{fmt, str::FromStr};

use crate::cli::verify_input_text;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CsvArgs {
    /// Name of the person to greet
    #[arg(short, long,  default_value_t = String::from("gtd"))]
    pub name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,

    #[arg(short, long, value_parser = verify_input_text)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for CsvArgs {
    async fn executor(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output.clone()
        } else {
            format!("output.{}", self.format)
        };
        crate::read_csv(&self.input, output, self.format)
    }
}

use std::path::Path;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long,  default_value_t = String::from("gtd"))]
    pub name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,

    #[arg(short, long, value_parser = verify_input_text)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,
}

fn verify_input_text(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("THE FILE NAME IS NOT EXISTS")
    }
}

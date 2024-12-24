use std::path::PathBuf;

use super::verify_input_path;
use clap::Parser;

#[derive(Debug, Parser)]
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

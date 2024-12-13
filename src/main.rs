use clap::Parser;
use handle_csv::read_csv;
use handle_csv::{Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let args: Opts = Opts::parse();

    match args.cmd {
        SubCommand::Csv(args) => {
            let output = if let Some(output) = args.output {
                output.clone()
            } else {
                format!("output.{}", args.format)
            };
            read_csv(&args.input, output, args.format)
        }
    }
}

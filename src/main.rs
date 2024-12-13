use clap::Parser;
use handle_csv::{process_genpassword, read_csv};
use handle_csv::{Opts, SubCommand};

fn main() {
    let args: Opts = Opts::parse();

    match args.cmd {
        SubCommand::Csv(args) => {
            let output = if let Some(output) = args.output {
                output.clone()
            } else {
                format!("output.{}", args.format)
            };
            let _ = read_csv(&args.input, output, args.format);
        }
        SubCommand::GenPass(args) => {
            let _ = process_genpassword(
                args.length,
                args.uppercase,
                args.lowercase,
                args.number,
                args.symbol,
            );
        }
    }
}

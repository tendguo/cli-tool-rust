use clap::Parser;
use handle_csv::{decode_base64, encode_base64, process_genpassword, read_csv, Base64Subcommand};

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
        SubCommand::Base64(args) => match args {
            Base64Subcommand::Encode(opts) => {
                let _ = encode_base64(&opts.input, opts.format);
            }
            Base64Subcommand::Decode(opts) => {
                let _ = decode_base64(&opts.input, opts.format);
            }
        },
    }
}

use anyhow::Ok;
use clap::Parser;
use handle_csv::{
    decode_base64, encode_base64, get_content, process_generate_key, process_genpassword,
    process_sign_text, process_sign_verify, read_csv, Base64Subcommand,
};

use handle_csv::{Opts, SubCommand, TextSubcommand};

fn main() -> Result<(), anyhow::Error> {
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
        SubCommand::Text(args) => match args {
            TextSubcommand::Generate(opts) => {
                let _ = process_generate_key(opts.format, opts.path);
            }
            TextSubcommand::Sign(opts) => {
                let key = get_content(&opts.key)?;
                let message = get_content(&opts.message)?;
                let _ = process_sign_text(opts.format, &key, &message);
            }
            TextSubcommand::Verify(opts) => {
                let key = get_content(&opts.key)?;
                let message = get_content(&opts.message)?;
                let result = process_sign_verify(opts.format, &key, &opts.sig, &message)?;
                if result {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
        },
    }
    Ok(())
}

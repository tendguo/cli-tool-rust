use std::process;

use clap::Parser;
use handle_csv::read_csv;
use handle_csv::Args;

fn main() {
    let args: Args = Args::parse();

    if let Err(err) = read_csv(&args.input, &args.output) {
        println!("error running {}", err);
        process::exit(1);
    }
}

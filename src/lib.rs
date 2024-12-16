mod cli;
mod process;

pub use cli::Base64Subcommand;
pub use cli::CsvArgs;
pub use cli::GenArgs;
pub use cli::Opts;
pub use cli::SubCommand;

pub use process::{decode_base64, encode_base64, process_genpassword, read_csv};
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

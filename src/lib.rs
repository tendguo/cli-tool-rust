mod cli;
mod process;

pub use cli::Base64Subcommand;
pub use cli::CsvArgs;
pub use cli::GenArgs;
pub use cli::Opts;
pub use cli::SubCommand;
pub use cli::TextSubcommand;

pub use process::{
    decode_base64, encode_base64, get_content, process_generate_key, process_genpassword,
    process_sign_text, process_sign_verify, read_csv,
};

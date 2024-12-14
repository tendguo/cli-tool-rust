mod cli;
mod input;
mod process;

pub use cli::CsvArgs;
pub use cli::GenArgs;
pub use input::Opts;

pub use input::SubCommand;
pub use process::process_genpassword;
pub use process::read_csv;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

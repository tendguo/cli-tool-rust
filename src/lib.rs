mod csv;
mod input;

pub use csv::read_csv;
pub use input::CsvArgs;
pub use input::Opts;
pub use input::OutputFormat;
pub use input::SubCommand;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

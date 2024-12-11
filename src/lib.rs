mod csv;
mod input;

pub use csv::read_csv;
pub use input::Args;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

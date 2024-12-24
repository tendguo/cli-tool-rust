use std::{fs::File, io::Read};
mod base64;
mod csv;
mod generate_password;
mod http_serve;
mod text;

use anyhow::Ok;
pub use base64::{decode_base64, encode_base64};
pub use csv::read_csv;
pub use generate_password::process_genpassword;
pub use http_serve::process_http_serve;
pub use text::{process_generate_key, process_sign_text, process_sign_verify};

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}

pub fn get_content(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

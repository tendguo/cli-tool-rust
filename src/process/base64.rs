use std::fs::File;
use std::io::Read;

use base64::prelude::*;

use crate::cli::Base64Format;

use base64::engine::general_purpose::URL_SAFE;

pub fn encode_base64(input: &str, format: Base64Format) -> Result<(), anyhow::Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encode = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE.encode(&buffer),
    };

    println!("{}", encode);
    Ok(())
}

pub fn decode_base64(input: &str, format: Base64Format) -> Result<(), anyhow::Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(&buffer)?,
        Base64Format::UrlSafe => URL_SAFE.decode(&buffer)?,
    };

    println!("{}", String::from_utf8(decode)?);
    Ok(())
}

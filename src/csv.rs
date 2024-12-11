use std::{error::Error, fs};

use serde_json::Value;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}

pub fn read_csv(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }
    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
    Ok(())
}

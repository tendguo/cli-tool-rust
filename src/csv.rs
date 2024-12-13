use std::fs;

use serde_json::Value;

use crate::input::OutputFormat;

pub fn read_csv(input: &str, output: String, format: OutputFormat) -> Result<(), anyhow::Error> {
    let mut rdr = csv::Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_json::to_string(&records)?,
    };
    fs::write(output, content)?;
    Ok(())
}

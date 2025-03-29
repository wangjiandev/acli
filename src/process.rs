use anyhow::{Ok, Result};
use csv::Reader;
use serde_json::Value;
use std::fs;

use crate::options::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(output, json)?;
        }
        OutputFormat::Yaml => {
            let yaml = serde_yaml::to_string(&ret)?;
            fs::write(output, yaml)?;
        }
    }
    Ok(())
}

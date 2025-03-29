use anyhow::{anyhow, Ok, Result};
use clap::{Args, Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "acli")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// csv to json
    Csv(CsvOptions),
}

/// # example
/// ```sh
/// cargo run -- csv -i input.csv -o output.json --header -d ","
/// ```
/// or build and run
/// ```sh
/// acli csv -i input.csv
/// ```
#[derive(Args, Debug)]
pub struct CsvOptions {
    /// input file
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    /// output file
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    /// header
    #[arg(long, default_value_t = true)]
    pub header: bool,
    /// delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String> {
    if !Path::new(filename).exists() {
        Err(anyhow!("File '{}' does not exist", filename))
    } else {
        Ok(filename.into())
    }
}

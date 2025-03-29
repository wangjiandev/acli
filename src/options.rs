use anyhow::{anyhow, Ok, Result};
use clap::{Args, Parser, Subcommand};
use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    str::FromStr,
};

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
    #[arg(short, long)]
    pub output: Option<String>,

    /// output format
    #[arg(short, long, value_parser = parser_format, default_value = "json")]
    pub format: OutputFormat,

    /// header
    #[arg(long, default_value_t = true)]
    pub header: bool,

    /// delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn verify_input_file(filename: &str) -> Result<String> {
    if !Path::new(filename).exists() {
        Err(anyhow!("File '{}' does not exist", filename))
    } else {
        Ok(filename.into())
    }
}

fn parser_format(format: &str) -> Result<OutputFormat> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(s: OutputFormat) -> Self {
        match s {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow!("Invalid output format: {}", s)),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

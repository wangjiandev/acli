use acli::{
    options::{Cli, Commands},
    process::process_csv,
};
use anyhow::{Ok, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Csv(options) => {
            process_csv(&options.input, &options.output)?;
        }
    }
    Ok(())
}

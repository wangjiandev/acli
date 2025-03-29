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
            let output = if let Some(output) = &options.output {
                output.clone()
            } else {
                format!("output.{}", options.format)
            };
            process_csv(&options.input, output, options.format)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_integration() {
        assert_eq!(10, 10);
    }
}

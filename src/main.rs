use anyhow::Result;
use clap::{Parser, Subcommand};

/// Manage Hugo versions
#[derive(Parser)]
struct Cli {
    /// The command to run
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Install a version of Hugo
    Install { version: String },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Install { version } => install(version),
    }
    Ok(())
}

fn install(version: std::string::String) {
    println!("Installing {}...", version)
}

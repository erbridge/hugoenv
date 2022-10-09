use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;
use std::path::Path;

const ROOT_ENV_VAR_NAME: &str = "HUGOENV_ROOT";
const ROOT_DEFAULT: &str = "~/.hugoenv";

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
        Command::Install { version } => install(version)?,
    }
    Ok(())
}

fn install(version: std::string::String) -> Result<()> {
    println!("Installing {version}...");
    ensure_root_dir()?;
    Ok(())
}

fn ensure_root_dir() -> Result<()> {
    let dir_string = env::var(ROOT_ENV_VAR_NAME).unwrap_or_else(|_| String::from(ROOT_DEFAULT));
    let expanded_dir_string = shellexpand::tilde(&dir_string);
    let dir = Path::new(expanded_dir_string.as_ref());
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

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
    ensure_versions_dir()?;
    Ok(())
}

fn ensure_versions_dir() -> Result<()> {
    let dir_string = root_dir() + "/versions";
    let dir = Path::new(&dir_string);
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn ensure_root_dir() -> Result<()> {
    let dir_string = root_dir();
    let dir = Path::new(&dir_string);
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn root_dir() -> String {
    let dir_string = &env::var(ROOT_ENV_VAR_NAME).unwrap_or_else(|_| String::from(ROOT_DEFAULT));
    shellexpand::tilde(dir_string).into_owned()
}

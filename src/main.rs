use anyhow::Result;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Install,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "install" => Ok(Command::Install),
            _ => anyhow::bail!("unknown command '{}'", input),
        }
    }
}

/// Manage Hugo versions.
#[derive(Parser)]
struct Cli {
    /// The command to run
    command: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let command = Command::from_str(&args.command)?;
    println!("Doing {:#?}!", command);
    Ok(())
}

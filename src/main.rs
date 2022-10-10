use anyhow::Result;
use clap::{Parser, Subcommand};

mod fs;
mod version;

use version::Version;

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

    /// Execute a command with the current version of Hugo
    Exec {
        #[command(subcommand)]
        program: Program,
    },
}

#[derive(Debug, Subcommand)]
enum Program {
    /// Run hugo with the current version of Hugo
    Hugo { args: Vec<String> },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Install { version } => {
            let ver = Version::new(version, &fs::root_dir());
            ver.install()?;
            Ok(())
        }

        Command::Exec { program } => {
            let local_version = Version::from_local(&fs::cwd(), &fs::root_dir());
            let local_version_name = local_version.name();

            if local_version_name == "system" {
                // TODO: Hand over to whatever's on the path.
                return Ok(());
            }

            println!("Running {program:?} with Hugo v{}...", local_version_name);
            Ok(())
        }
    }
}

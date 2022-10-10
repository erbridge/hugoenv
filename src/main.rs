use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process;

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
    Hugo {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
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

            match program {
                Program::Hugo { args } => {
                    let prog = if local_version.is_system() {
                        "hugo"
                    } else if local_version.is_installed() {
                        local_version.bin_file().to_str().unwrap()
                    } else {
                        panic!("Hugo v{} is not installed", local_version.name());
                    };
                    process::Command::new(prog)
                        .args(args)
                        .status()
                        .expect("hugo should have run successfully");
                }
            }
            Ok(())
        }
    }
}

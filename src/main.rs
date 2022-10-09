use clap::Parser;

/// Manage Hugo versions.
#[derive(Parser)]
struct Cli {
    /// The command to run
    command: String,
}

fn main() {
    let args = Cli::parse();

    println!("Doing {}!", args.command)
}

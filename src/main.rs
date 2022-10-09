use anyhow::Result;
use clap::{Parser, Subcommand};
use flate2::read::GzDecoder;
use std::env;
use std::path::Path;
use tar::Archive;

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
        Command::Install { version } => install(&version)?,
    }
    Ok(())
}

fn install(version: &String) -> Result<()> {
    println!("Installing {version}...");
    let version_dir = ensure_versions_dir()?;
    if version_is_installed(version, &version_dir) {
        println!("Version {version} is already installed!");
    } else {
        download_version(version, &version_dir)?;
    }
    Ok(())
}

fn ensure_versions_dir() -> Result<String> {
    let dir_string = root_dir() + "/versions";
    let dir = Path::new(&dir_string);
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(String::from(dir.to_str().unwrap()))
}

fn ensure_root_dir() -> Result<String> {
    let dir_string = root_dir();
    let dir = Path::new(&dir_string);
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(String::from(dir.to_str().unwrap()))
}

fn root_dir() -> String {
    let dir_string = &env::var(ROOT_ENV_VAR_NAME).unwrap_or_else(|_| String::from(ROOT_DEFAULT));
    shellexpand::tilde(dir_string).into_owned()
}

fn hugo_bin(version: &String, versions_dir: &String) -> String {
    format!("{versions_dir}/{version}")
}

fn version_is_installed(version: &String, versions_dir: &String) -> bool {
    let hugo_bin_string = hugo_bin(version, versions_dir);
    let hugo_bin = Path::new(&hugo_bin_string);
    hugo_bin.exists()
}

fn download_version(version: &String, versions_dir: &String) -> Result<()> {
    let url = format!("https://github.com/gohugoio/hugo/releases/download/v{version}/hugo_{version}_darwin-universal.tar.gz");
    let response = reqwest::blocking::get(url)?;
    extract_version(response, version, versions_dir)?;
    Ok(())
}

fn extract_version(
    tar_gz: impl std::io::Read,
    version: &String,
    versions_dir: &String,
) -> Result<()> {
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(format!("{versions_dir}/{version}"))?;
    Ok(())
}

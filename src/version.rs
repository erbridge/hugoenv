use std::path::{Path, PathBuf};

use anyhow::Result;
use flate2::read::GzDecoder;
use reqwest::blocking as reqwest;
use tar::Archive;

use super::fs;

pub struct Version {
  name: String,
  versions_dir: PathBuf,
  version_dir: PathBuf,
  bin_file: PathBuf,
}

impl Version {
  pub fn new(name: String, root_dir: &Path) -> Version {
    let versions_dir = root_dir.join("versions");
    let version_dir = versions_dir.join(&name);
    let bin_file = version_dir.join("hugo");

    Version {
      name,
      versions_dir,
      version_dir,
      bin_file,
    }
  }

  pub fn install(&self) -> Result<()> {
    println!("Installing {}...", self.name);
    fs::ensure_dir(&self.versions_dir)?;
    if self.is_installed() {
      println!("Version {} is already installed!", self.name);
    } else {
      self.download()?;
    }
    Ok(())
  }

  fn is_installed(&self) -> bool {
    self.bin_file.exists()
  }

  fn download(&self) -> Result<()> {
    let url = format!(
      "https://github.com/gohugoio/hugo/releases/download/v{}/hugo_{}_darwin-universal.tar.gz",
      self.name.replace("extended_", ""),
      self.name
    );
    let response = reqwest::get(url)?;
    self.extract(response)?;
    Ok(())
  }

  fn extract(&self, tar_gz: impl std::io::Read) -> Result<()> {
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&self.version_dir)?;
    Ok(())
  }
}

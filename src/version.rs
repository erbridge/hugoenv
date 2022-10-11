use std::env::consts::ARCH;
use std::path::{Path, PathBuf};

use anyhow::Result;
use flate2::read::GzDecoder;
use reqwest::blocking as reqwest;
use semver::VersionReq;
use tar::Archive;

use super::fs;

pub struct Version {
  name: String,
  version: semver::Version,
  versions_dir: PathBuf,
  version_dir: PathBuf,
  bin_file: PathBuf,
}

impl Version {
  pub fn new(name: String, root_dir: &Path) -> Version {
    let ver = semver::Version::parse(&name.replace("extended_", "")).unwrap();
    let versions_dir = root_dir.join("versions");
    let version_dir = versions_dir.join(&name);
    let bin_file = version_dir.join("hugo");

    Version {
      name,
      version: ver,
      versions_dir,
      version_dir,
      bin_file,
    }
  }

  pub fn from_local(cwd: &Path, root_dir: &Path) -> Version {
    let name = match fs::search_up(".hugo-version", cwd) {
      Some(path) => std::fs::read_to_string(path).unwrap().trim().to_string(),
      None => String::from("system"),
    };

    Version::new(name, root_dir)
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn bin_file(&self) -> &PathBuf {
    &self.bin_file
  }

  pub fn is_system(&self) -> bool {
    self.name == "system"
  }

  pub fn is_installed(&self) -> bool {
    self.bin_file.exists()
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

  fn download(&self) -> Result<()> {
    let url = self.download_url()?;
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

  fn download_url(&self) -> Result<String> {
    let name = &self.name;
    let unextended_name = name.replace("extended_", "");

    let url = if self.is_matched_by("<0.102.0") {
      match ARCH {
        "x86_64" => format!(
          "https://github.com/gohugoio/hugo/releases/download/v{}/hugo_{}_macOS-64bit.tar.gz",
          unextended_name, name
        ),
        "aarch64" => format!(
          "https://github.com/gohugoio/hugo/releases/download/v{}/hugo_{}_macOS-ARM64.tar.gz",
          unextended_name, name
        ),
        _ => return Err(anyhow::anyhow!("{} is not supported", ARCH)),
      }
    } else if self.is_matched_by("<0.103.0") {
      format!(
        "https://github.com/gohugoio/hugo/releases/download/v{}/hugo_{}_macOS-universal.tar.gz",
        unextended_name, name
      )
    } else {
      format!(
        "https://github.com/gohugoio/hugo/releases/download/v{}/hugo_{}_darwin-universal.tar.gz",
        unextended_name, name
      )
    };

    Ok(url)
  }

  fn is_matched_by(&self, version_constraint: &str) -> bool {
    VersionReq::parse(version_constraint)
      .unwrap()
      .matches(&self.version)
  }
}

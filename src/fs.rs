use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

const ROOT_ENV_VAR_NAME: &str = "HUGOENV_ROOT";
const ROOT_DEFAULT: &str = "~/.hugoenv";

pub fn ensure_dir(dir: &Path) -> Result<()> {
  if !dir.exists() {
    fs::create_dir_all(dir)?;
  }
  Ok(())
}

pub fn search_up(filename: &str, start: &Path) -> Option<PathBuf> {
  let mut dir = start.to_owned();
  loop {
    let file = dir.join(filename);
    if file.exists() {
      return Some(file);
    }
    dir = match dir.parent() {
      Some(d) => d,
      None => return None,
    }
    .to_path_buf();
  }
}

pub fn root_dir() -> PathBuf {
  let dir_string = &env::var(ROOT_ENV_VAR_NAME).unwrap_or_else(|_| String::from(ROOT_DEFAULT));
  let expanded_dir_string = shellexpand::tilde(dir_string).into_owned();
  PathBuf::from(expanded_dir_string)
}

pub fn cwd() -> PathBuf {
  env::current_dir().unwrap()
}

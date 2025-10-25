use std::env::home_dir;
use std::path::PathBuf;

pub fn kanata_dir() -> PathBuf {
  home_dir()
    .expect("failed to retrieve the home directory")
    .join(".kanata")
}

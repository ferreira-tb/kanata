use std::env::home_dir;
use std::path::PathBuf;

pub fn kanata_dir() -> PathBuf {
  home_dir().unwrap().join(".kanata")
}

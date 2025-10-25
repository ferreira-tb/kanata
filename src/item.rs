use crate::path::kanata_dir;
use anyhow::Result;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode, utf8_percent_encode};
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct Item {
  name: String,
  path: PathBuf,
}

impl Item {
  pub fn read(path: PathBuf) -> Option<Self> {
    let name = path.file_name()?.to_str()?;
    Some(Self { name: name.to_owned(), path })
  }

  pub async fn read_dir() -> Result<Vec<Self>> {
    let mut items = Vec::new();

    let dir = kanata_dir();
    fs::create_dir_all(&dir).await?;
    let mut entries = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
      if entry.file_type().await?.is_file() {
        let path = entry.path();
        if let Some(item) = Self::read(path) {
          items.push(item)
        }
      }
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(items)
  }

  pub fn decode(name: &[u8]) -> Result<Option<Self>> {
    let name = percent_decode(name).decode_utf8()?;
    let path = kanata_dir().join(name.as_ref());
    Ok(Self::read(path))
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn encode(&self) -> String {
    utf8_percent_encode(&self.name, NON_ALPHANUMERIC).to_string()
  }
}

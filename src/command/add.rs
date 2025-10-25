use super::Command;
use crate::path::kanata_dir;
use anyhow::{Result, bail};
use clap::Args;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs;

#[derive(Args, Debug)]
pub struct Add {
  #[arg(short = 'p', long)]
  path: PathBuf,
  #[arg(short = 'n', long)]
  name: Option<String>,
}

impl Command for Add {
  async fn execute(self) -> Result<()> {
    let Some(name) = self.name.or_else(|| {
      self
        .path
        .file_name()
        .and_then(OsStr::to_str)
        .map(ToOwned::to_owned)
    }) else {
      bail!("missing file name");
    };

    let target = kanata_dir().join(name);
    fs::copy(self.path, target).await?;

    Ok(())
  }
}

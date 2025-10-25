use super::Command;
use crate::path::kanata_dir;
use anyhow::Result;
use clap::Args;
use tokio::fs;

#[derive(Args, Debug)]
pub struct Clear;

impl Command for Clear {
  async fn execute(self) -> Result<()> {
    let dir = kanata_dir();
    if fs::try_exists(&dir).await? {
      fs::remove_dir(dir).await?;
    }

    Ok(())
  }
}

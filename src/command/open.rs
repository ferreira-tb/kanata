use super::Command;
use crate::path::kanata_dir;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct Open {
  #[arg(short = 'n', long)]
  name: Option<String>,
}

impl Command for Open {
  async fn execute(self) -> Result<()> {
    let mut path = kanata_dir();
    if let Some(name) = self.name {
      path.push(name);
    }

    open::that_detached(path)?;

    Ok(())
  }
}

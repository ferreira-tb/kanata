use super::Command;
use crate::item::Item;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct List {
  #[arg(short = 'a', long)]
  absolute: bool,
}

impl Command for List {
  async fn execute(self) -> Result<()> {
    let items = Item::read_dir().await?;
    for item in items {
      if self.absolute {
        println!("{}", item.path().to_string_lossy());
      } else {
        println!("{}", item.name());
      }
    }

    Ok(())
  }
}

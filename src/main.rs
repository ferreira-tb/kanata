mod command;
mod item;
mod path;

use anyhow::Result;
use clap::Parser;
use command::Command;

#[derive(Debug, Parser)]
#[command(name = "kanata")]
#[command(version, about, long_about = None)]
enum Cli {
  Add(command::Add),
  List(command::List),
  Open(command::Open),
  Serve(command::Serve),
}

#[tokio::main]
async fn main() -> Result<()> {
  match Cli::parse() {
    Cli::Add(cmd) => cmd.execute().await,
    Cli::List(cmd) => cmd.execute().await,
    Cli::Open(cmd) => cmd.execute().await,
    Cli::Serve(cmd) => cmd.execute().await,
  }
}

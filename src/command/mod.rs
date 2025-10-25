mod add;
mod list;
mod open;
mod serve;

use anyhow::Result;

pub use add::Add;
pub use list::List;
pub use open::Open;
pub use serve::Serve;

pub trait Command {
  async fn execute(self) -> Result<()>;
}

mod add;
mod clear;
mod list;
mod open;
mod serve;

use anyhow::Result;

pub use add::Add;
pub use clear::Clear;
pub use list::List;
pub use open::Open;
pub use serve::Serve;

pub trait Command {
  async fn execute(self) -> Result<()>;
}

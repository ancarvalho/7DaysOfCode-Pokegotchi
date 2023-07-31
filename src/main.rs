mod config;
mod domain;
mod infra;
mod presenter;

use anyhow::Result;

use crate::domain::{utils::header::pokegotchi, enums::screen_possibilities::ScreenPossibilities};

#[tokio::main]
async fn main() -> Result<()> {
  print!("{}", pokegotchi);
  
  let _ = ScreenPossibilities::start()?;

  Ok(())
}

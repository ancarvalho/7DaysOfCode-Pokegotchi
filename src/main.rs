mod config;
mod domain;
mod infra;
mod presenter;

use anyhow::Result;

use crate::domain::{utils::header::POKEGOTCHI, enums::screen_possibilities::ScreenPossibilities, entities::data::Data};

#[tokio::main]
async fn main() -> Result<()> {
  // data geting update to slow
  let mut data = Data::new();
  print!("{}", POKEGOTCHI);
  
  let _ = ScreenPossibilities::start(&mut data)?;

  Ok(())
}

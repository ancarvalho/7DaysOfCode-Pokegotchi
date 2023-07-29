mod config;
mod domain;
mod infra;
mod presenter;

use std::ops::ControlFlow;

use anyhow::Result;

use crate::{
  domain::utils::header::pokegotchi,
  presenter::pages::{
    home::home_page, pokemon_adoption::pokemon_adoption, pokemon_choose::pokemon_choose,
  },
};

#[tokio::main]
async fn main() -> Result<()> {
  print!("{}", pokegotchi);

  let result = home_page()?;
  println!("Olá {}", result);

  let pokemon = pokemon_choose()?;
  pokemon_adoption(pokemon).await?;

  // let mut r = ControlFlow::Continue(_);

  // while let ControlFlow::Continue(_) = r {}

  Ok(())
}

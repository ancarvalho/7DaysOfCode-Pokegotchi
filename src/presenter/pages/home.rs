use anyhow::Result;
use std::io;

use crate::{domain::{enums::screen_possibilities::ScreenPossibilities, repositories::PokegotchiRepositoryAbstract}, infra::repo::pokegotchi_repo_impl::PokegotchiRepoImpl};

pub async fn home_page(pokegochi_repo: &PokegotchiRepoImpl) -> Result<ScreenPossibilities>  {
  let mut username = String::new();
  let stdin = io::stdin();

  loop {
    println!("Seu Nome: (minimo 2 caracters)");
    stdin.read_line(&mut username)?;

    if username.len() >= 3 {
      println!("{}", username.len());
      break;
    }
  }
  pokegochi_repo.create_trainer(username).await?;

  Ok(ScreenPossibilities::PokemonChoosePage)
  
}

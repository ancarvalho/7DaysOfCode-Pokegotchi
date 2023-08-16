use anyhow::{Ok, Result};
use std::io;

use crate::{domain::{

  enums::screen_possibilities::ScreenPossibilities, repositories::PokegotchiRepositoryAbstract,
}, infra::repo::pokegotchi_repo_impl::PokegotchiRepoImpl};

pub async fn my_pokemons_page(pokegochi_repo: &PokegotchiRepoImpl) -> Result<ScreenPossibilities> {
  println!("Yo {:#?}", pokegochi_repo.get_trainer_name().await?);
  let pokemons_size = pokegochi_repo.get_pokemons().await?;

  for i in 0..pokemons_size.len() {
    println!(
      "Para Selecionar {}, digite {}",
      pokemons_size[i].name,
      i.to_string()
    )
  }
  println!("Para Adotar Outro Pokemon digite 3",);
  println!("Para Sair digite 4",);

  loop {
    let mut index = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index)?;

    let parsed_index: usize = match index.trim().parse::<usize>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    if Vec::from_iter(0..pokemons_size.len()).contains(&parsed_index) {
      return Ok(ScreenPossibilities::MyPokomonDetailsPage {
        index: pokemons_size[parsed_index].id.try_into()?,
      });
    }

    return match parsed_index {
      3 => Ok(ScreenPossibilities::PokemonChoosePage),
      4 => Ok(ScreenPossibilities::Quit),
      _ => continue,
    };
    // TODO Check (may cause error)
  }
}

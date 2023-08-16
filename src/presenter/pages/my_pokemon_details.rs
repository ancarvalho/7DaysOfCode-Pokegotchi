use crate::{
  domain::{
    enums::screen_possibilities::ScreenPossibilities, repositories::PokegotchiRepositoryAbstract,
  },
  infra::repo::pokegotchi_repo_impl::PokegotchiRepoImpl,
};
use anyhow::{Ok, Result};
use std::io;

pub async fn my_pokemons_details_page(
  pokegochi_repo: &PokegotchiRepoImpl,
  index: &usize,
) -> Result<ScreenPossibilities> {
  // print!("{}", pokemon.ascii_image);

  // let mut mutable_pokemon = my_pokemon.clone();

  let mut my_pokemon = pokegochi_repo
    .get_pokemon(index.clone().try_into()?)
    .await?;
  println!("Digite 0 para Voltar");
  println!("Digite 1 Brincar com {}", my_pokemon.name);
  println!("Digite 2 Para Alimentar {}", my_pokemon.name);
  println!("Digite 3 Para Ver Felicidade {}", my_pokemon.name);
  println!("Digite 4 Para Ver Fome {}", my_pokemon.name);
  println!("Digite 5 Para Abandonar {}", my_pokemon.name);

  println!("Digite qualquer outro numero pra sair");

  'pokemon_adoption: loop {
    let mut index_str = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index_str)?;

    let parsed_index: i8 = match index_str.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    match parsed_index {
      0 => return Ok(ScreenPossibilities::MyPokemonsPage),
      1 => {
        my_pokemon.play_with_pokemon().check_pokemon_happiness();
        pokegochi_repo
          .update_pokemon_played(index.clone().try_into()?)
          .await?;
        return Ok(ScreenPossibilities::MyPokomonDetailsPage { index: index.clone() });
      }
      2 => {
        my_pokemon.feed_pokemon().check_pokemon_hungry();
        pokegochi_repo
          .update_pokemon_feed(index.clone().try_into()?)
          .await?;
        return Ok(ScreenPossibilities::MyPokomonDetailsPage { index: index.clone() });
      }
      3 => {
        my_pokemon.check_pokemon_happiness();
        continue;
      }
      4 => {
        my_pokemon.check_pokemon_hungry();
        continue;
      }
      5 => {
        pokegochi_repo
          .remove_pokemon(index.clone().try_into()?)
          .await?;
        return Ok(ScreenPossibilities::MyPokemonsPage);
      }

      _ => break 'pokemon_adoption,
    };
  }
  Ok(ScreenPossibilities::Quit)
}

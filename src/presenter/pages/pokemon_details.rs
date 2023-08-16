use crate::{domain::{
  entities::pokemon_details::PokemonDetails,
  enums::screen_possibilities::ScreenPossibilities, repositories::PokegotchiRepositoryAbstract,
}, infra::repo::{pokegotchi_repo_impl::PokegotchiRepoImpl, get_pokemon_info::get_pokemon_info}};
use anyhow::{Ok, Result};
use std::io;

pub async fn pokemon_details_page(pokemon: &PokemonDetails, pokegochi_repo: &PokegotchiRepoImpl) -> Result<ScreenPossibilities> {

  let pokemon_info = get_pokemon_info(pokemon.pokedex_name.clone()).await?;
  println!("{}", pokemon_info);
  println!("Digite 1 Voltar");
  println!("Digite 2 Para Escolher {}", pokemon.name.clone());



  loop {
    let mut index = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index)?;

    let parsed_index: i8 = match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    // let parsed_index: i8 = index.trim().parse::<i8>().expect("Digite um Numero");
    return match parsed_index {
      1 => Ok(ScreenPossibilities::PokemonAdoptionPage {
        pokemon: pokemon.clone(),
      }),
      2 => {
        pokegochi_repo.add_pokemon(pokemon.clone()).await?;
        Ok(ScreenPossibilities::MyPokemonsPage)
      
      }
      _ => Ok(ScreenPossibilities::Quit),
    };
    // TODO Check (may cause error)
  }
  // Ok(ScreenPossibilities::Quit)
}

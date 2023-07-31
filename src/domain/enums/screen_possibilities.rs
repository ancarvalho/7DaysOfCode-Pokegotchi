use crate::{
  domain::entities::{my_pokemon::MyPokemon, pokemon_details::PokemonDetails},
  presenter::pages::{home::home_page, pokemon_choose:: pokemon_choose_page, pokemon_adoption::pokemon_adoption_page, pokemon_details::pokemon_details_page, my_pokemon::my_pokemon_page},
};

use anyhow::Result;

#[derive(Debug)]
pub enum ScreenPossibilities {
  HomePage ,
  PokemonChoosePage {name: String},
  PokemonAdoptionPage { pokemon: PokemonDetails },
  PokemonDetailsPage { pokemon: PokemonDetails },
  MyPokemonPage { my_pokemon:   MyPokemon},
  Quit,
}

impl ScreenPossibilities {
  pub fn match_possibilities(&self) -> Result<ScreenPossibilities>{
    match &self {
      ScreenPossibilities::HomePage  => home_page()?.match_possibilities(),
      ScreenPossibilities::PokemonChoosePage { name } => pokemon_choose_page(name.clone())?.match_possibilities(),
      ScreenPossibilities::PokemonAdoptionPage { pokemon } => pokemon_adoption_page(pokemon)?.match_possibilities(),
      ScreenPossibilities::PokemonDetailsPage { pokemon } => pokemon_details_page(pokemon)?.match_possibilities(),
      ScreenPossibilities::MyPokemonPage {  my_pokemon } => my_pokemon_page( my_pokemon)?.match_possibilities(),
      ScreenPossibilities::Quit => panic!("User Exited Application"),
    }
  }

  pub fn start() -> Result<ScreenPossibilities> {
    ScreenPossibilities::HomePage.match_possibilities()
  }
}

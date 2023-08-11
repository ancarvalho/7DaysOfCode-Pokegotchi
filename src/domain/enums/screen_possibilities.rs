use crate::{
  domain::entities::{pokemon_details::PokemonDetails, data::Data},
  presenter::pages::{home::home_page, pokemon_choose:: pokemon_choose_page, pokemon_adoption::pokemon_adoption_page, pokemon_details::pokemon_details_page,  my_pokemons::my_pokemons_page, my_pokemon_details::my_pokemons_details_page},
};

use anyhow::Result;

#[derive(Debug)]
pub enum ScreenPossibilities {
  HomePage ,
  PokemonChoosePage ,
  PokemonAdoptionPage { pokemon: PokemonDetails },
  PokemonDetailsPage { pokemon: PokemonDetails },
  MyPokemonsPage ,
  MyPokomonDetailsPage {index: usize},
  Quit,
}

impl ScreenPossibilities {
  pub fn match_possibilities(&self, data: &mut Data) -> Result<ScreenPossibilities>{
    match &self {
      ScreenPossibilities::HomePage  => home_page(data)?.match_possibilities(data),
      ScreenPossibilities::PokemonChoosePage => pokemon_choose_page(data)?.match_possibilities(data),
      ScreenPossibilities::PokemonAdoptionPage { pokemon } => pokemon_adoption_page(pokemon, data)?.match_possibilities(data),
      ScreenPossibilities::PokemonDetailsPage { pokemon } => pokemon_details_page(pokemon, data)?.match_possibilities(data),
      ScreenPossibilities::MyPokemonsPage  => my_pokemons_page( data)?.match_possibilities(data),
      ScreenPossibilities::MyPokomonDetailsPage { index }  => my_pokemons_details_page(data, index)?.match_possibilities(data),
      ScreenPossibilities::Quit => panic!("User Exited Application"),
    }
  }

  pub fn start(data : &mut Data) -> Result<ScreenPossibilities> {
    ScreenPossibilities::HomePage.match_possibilities(data)
  }
}

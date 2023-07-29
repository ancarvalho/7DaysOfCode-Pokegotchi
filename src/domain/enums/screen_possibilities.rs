use crate::{domain::entities::{pokemon_details::PokemonDetails, my_pokemon::MyPokemon}, presenter::pages::home::home_page};

#[derive(Debug)]
pub enum ScreenPossibilities {
  Home{name: String},
  AvailablePokemons,
  PokemonAdoption{pokemon: PokemonDetails},
  PokemonDetails{pokemon: PokemonDetails},
  MyPokemon{my_pokemon: MyPokemon},
}


impl ScreenPossibilities {
    pub fn match_possibilities(&self) {
      match &self {
        ScreenPossibilities::Home { name } => {home_page()},
        ScreenPossibilities::AvailablePokemons => todo!(),
        ScreenPossibilities::PokemonAdoption { pokemon } => todo!(),
        ScreenPossibilities::PokemonDetails { pokemon } => todo!(),
        ScreenPossibilities::MyPokemon { my_pokemon } => todo!(),
    }
    }
}
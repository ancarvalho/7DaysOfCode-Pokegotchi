use crate::domain::{
  entities::pokemon_details::PokemonDetails,
  // utils::ascii_pokemons::{bulbasour_ascii, pikachu_ascii, squirtle_ascii},
};

#[derive(Debug)]
pub enum AvailablePokemons {
  Bulbasaur,
  Pikachu,
  Squirtle,
}

impl AvailablePokemons {
  pub fn get_pokemon(&self) -> PokemonDetails {
    let pokemon = match &self {
      AvailablePokemons::Pikachu => PokemonDetails {
        name: "Pikachu".to_string(),
        // ascii_image: pikachu_ascii,
        pokedex_name: "pikachu".to_string(),
      },
      AvailablePokemons::Bulbasaur => PokemonDetails {
        name: "Bulbasauro".to_string(),
        // ascii_image: bulbasour_ascii,
        pokedex_name: "bulbasaur".to_string(),
      },
      AvailablePokemons::Squirtle => PokemonDetails {
        name: "Squirtle".to_string(),
        // ascii_image: squirtle_ascii,
        pokedex_name: "squirtle".to_string(),
      },
    };

    pokemon
  }
}

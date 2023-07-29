use crate::domain::entities::{pokemon::Pokemon, pokemon_simplified::PokemonSimplified};

impl Pokemon {
  pub fn transform(self) -> PokemonSimplified {
    let abilities = self.abilities.iter().map(|a| a.ability.name.clone()).collect();
    PokemonSimplified{name: self.name, height: self.height,  weight: self.weight, abilities}
  }
}
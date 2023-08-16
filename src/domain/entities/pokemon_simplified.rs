use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSimplified {
  pub name: String,
  pub height: i8,
  pub weight: i16,
  pub abilities: Vec<String>
}

impl Display for PokemonSimplified {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "O Pokemon {}, Tem {} de Altura e {} de peso e habilidades {}", self.name, self.height, self.weight, self.abilities.join(", "))    
  }
}


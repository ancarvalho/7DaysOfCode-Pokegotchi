use super::{my_pokemon::MyPokemon, pokemon_details::PokemonDetails};

#[derive(Debug, Clone)]
pub struct Data {
  pub name: Option<String>,
  pub pokemons: Vec<MyPokemon>,
}

impl Data {
  pub fn new() -> Data {
    Data {
      name: None,
      pokemons: vec![],
    }
  }

  pub fn change_name(&mut self, name: String) -> &Self {
    self.name = Some(name);

    return self;
  }

  pub fn add_pokemon(&mut self, pokemon: PokemonDetails) -> &Self {
    let pokemon = MyPokemon {
      pokemon: pokemon,
      last_feed: None,
      last_played_with: None,
    };

    if self.pokemons.len() <= 3 {
      self.pokemons.push(pokemon);
    }

    return self;
  }

  pub fn remove_pokemon(&mut self, index: usize) -> &Self {
    self.pokemons.swap_remove(index);

    return self;
  }


  pub fn get_pokemon(&mut self, index: usize) -> &mut MyPokemon {
    &mut self.pokemons[index]
  } 
}

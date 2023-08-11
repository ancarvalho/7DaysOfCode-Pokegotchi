use std::io;

use anyhow::{Ok, Result};

use crate::domain::{enums::{
  inital_pokemons::AvailablePokemons, screen_possibilities::ScreenPossibilities,
}, entities::data::Data};

pub fn pokemon_choose_page(data: &Data) -> Result<ScreenPossibilities> {
  println!("Ola {:#?}", &data.name);

  println!("Digite 1 Para Escolher Bubasauro");
  println!("Digite 2 Para Escolher Pikachu");
  println!("Digite 3 Para Escolher Squirtle");
  println!("Digite qualquer outro numero pra sair");


  // TODO analyse error if index is moved outside loop there's a bug on loop
  'pokemon_choose_page: loop {
    let mut index = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index)?;

    return match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => Ok(encapsulate_on_screen(match_pokemon(num))),
      core::result::Result::Err(_) => continue 'pokemon_choose_page,
    };


  }
}

pub fn encapsulate_on_screen(pokemon: AvailablePokemons) -> ScreenPossibilities {
  ScreenPossibilities::PokemonAdoptionPage {
    pokemon: pokemon.get_pokemon(),
  }
}

pub fn match_pokemon(index: i8) -> AvailablePokemons {
  return match index {
    1 => AvailablePokemons::Bulbasaur,
    2 => AvailablePokemons::Pikachu,
    3 => AvailablePokemons::Squirtle,
    _ => panic!(),
  };
}

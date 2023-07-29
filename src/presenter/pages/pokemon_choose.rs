use std::io;

use anyhow::{Ok, Result};

use crate::domain::{enums::inital_pokemons::AvailablePokemons, entities::pokemon_details::PokemonDetails};

pub fn pokemon_choose() -> Result<PokemonDetails> {
  let mut index = String::new();
  let stdin = io::stdin();
  let pokemon: AvailablePokemons;

  println!("Digite 1 Para Escolher Bubasauro");
  println!("Digite 2 Para Escolher Pikachu");
  println!("Digite 3 Para Escolher Squirtle");
  println!("Digite qualquer outro numero pra sair");

  loop {
    stdin.read_line(&mut index)?;

    let parsed_index: i8 = match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    // let parsed_index: i8 = index.trim().parse::<i8>().expect("Digite um Numero");
    pokemon = match parsed_index {
      1 => AvailablePokemons::Bulbasaur,
      2 => AvailablePokemons::Pikachu,
      3 => AvailablePokemons::Squirtle,
      _ => panic!(),
    };
    break;
  }
  // print!("{}", pokemon.get_pokemon().ascii_image);
  Ok(pokemon.get_pokemon())
}

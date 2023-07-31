use crate::domain::{
  entities::{my_pokemon::MyPokemon, pokemon_details::PokemonDetails},
  enums::screen_possibilities::ScreenPossibilities,
};
use anyhow::{Ok, Result};
use std::io;

pub fn pokemon_adoption_page(pokemon: &PokemonDetails) -> Result<ScreenPossibilities> {
  // print!("{}", pokemon.ascii_image);

  println!("Digite 1 Para Ver Mais Detalhes");
  println!("Digite 2 Para Escolher {}", pokemon.name);
  // println!("Digite 3 Para Voltar");
  println!("Digite qualquer outro numero pra sair");

  loop {
    let mut index = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index)?;

    let parsed_index: i8 = match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    return match parsed_index {
      1 => Ok(ScreenPossibilities::PokemonDetailsPage {
        pokemon: pokemon.clone(),
      }),

      2 => {
        let my = MyPokemon::new(pokemon.clone());
        return Ok(ScreenPossibilities::MyPokemonPage { my_pokemon: my });
        // break 'pokemon_adoption;
      }
      3 => continue,
      _ => Ok(ScreenPossibilities::Quit),
    };

    // return  y;
  }
}

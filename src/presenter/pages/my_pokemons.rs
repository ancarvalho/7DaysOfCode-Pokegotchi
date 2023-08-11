use anyhow::{Ok, Result};
use std::io;

use crate::domain::{
  entities::data::Data,
  enums::screen_possibilities::ScreenPossibilities,
};

pub fn my_pokemons_page(data: &mut Data) -> Result<ScreenPossibilities> {
  println!("Yo {:#?}", &data.name);
  let pokemons_size = data.pokemons.len().clone();

  for i in 0..pokemons_size {
    println!(
      "Para Selecionar {}, digite {}",
      &data.get_pokemon(i).pokemon.name,
      i.to_string()
    )
  }
  println!("Para Adotar Outro Pokemon digite 3",);
  println!("Para Sair digite 4",);

  loop {
    let mut index = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index)?;

    let parsed_index: usize = match index.trim().parse::<usize>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    if Vec::from_iter(0..pokemons_size).contains(&parsed_index) {
      return Ok(ScreenPossibilities::MyPokomonDetailsPage {
        index: parsed_index,
      });
    }

    return match parsed_index {
      3 => Ok(ScreenPossibilities::PokemonChoosePage),
      4 => Ok(ScreenPossibilities::Quit),
      _ => continue,
    };
    // TODO Check (may cause error)
  }
}

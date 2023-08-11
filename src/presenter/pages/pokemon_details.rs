use crate::domain::{
  entities::{pokemon_details::PokemonDetails, data::Data},
  enums::screen_possibilities::ScreenPossibilities,
};
use anyhow::{Ok, Result};
use std::io;

pub fn pokemon_details_page(pokemon: &PokemonDetails, data: &mut Data) -> Result<ScreenPossibilities> {
  // print!("{}", pokemon.get_pokemon().ascii_image);

  // let pokemon_info = get_pokemon_info(pokemon.pokedex_name.clone()).await?;
  // println!("{}", pokemon_info);
  println!("Digite 1 Voltar");
  println!("Digite 2 Para Escolher {}", pokemon.name.clone());

  loop {
    let mut index = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index)?;

    let parsed_index: i8 = match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    // let parsed_index: i8 = index.trim().parse::<i8>().expect("Digite um Numero");
    return match parsed_index {
      1 => Ok(ScreenPossibilities::PokemonAdoptionPage {
        pokemon: pokemon.clone(),
      }),
      2 => {
        data.add_pokemon(pokemon.clone());
        Ok(ScreenPossibilities::MyPokemonsPage)
      
      }
      _ => Ok(ScreenPossibilities::Quit),
    };
    // TODO Check (may cause error)
  }
  // Ok(ScreenPossibilities::Quit)
}

use crate::{domain::entities::{pokemon_details::PokemonDetails, my_pokemon::MyPokemon}, infra::repo::get_pokemon_info::get_pokemon_info, presenter::pages::{pokemon_adoption::pokemon_adoption, my_pokemon::my_pokemon}};
use anyhow::{Ok, Result};
use std::io;

pub async fn pokemon_details(pokemon: PokemonDetails) -> Result<()> {
  // print!("{}", pokemon.get_pokemon().ascii_image);

  let pokemon_info  = get_pokemon_info(pokemon.pokedex_name.clone()).await?;
  println!("{}", pokemon_info);
  println!("Digite 1 Voltar");
  println!("Digite 2 Para Escolher {}", pokemon.name);
  

  let mut index = String::new();
  let stdin = io::stdin();
  
  'pokemon_details: loop {
    stdin.read_line(&mut index)?;

    let parsed_index: i8 = match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    // let parsed_index: i8 = index.trim().parse::<i8>().expect("Digite um Numero");
    match parsed_index {
      2 => {
        let my = &mut MyPokemon::new(pokemon);
        let _ = my_pokemon(my);
        // break 'pokemon_details;
      },
      1 => {
        let _ = pokemon_adoption(pokemon.clone());
        // break 'pokemon_details;
      },
      _ => panic!(),
    };
    break 'pokemon_details;
  }
  Ok(())
}

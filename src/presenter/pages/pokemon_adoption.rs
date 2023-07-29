use crate::{
  domain::entities::{my_pokemon::MyPokemon, pokemon_details::PokemonDetails},
  presenter::pages::{my_pokemon::my_pokemon, pokemon_details::pokemon_details},
};
use anyhow::{Ok, Result};
use std::io;

pub async fn pokemon_adoption(pokemon: PokemonDetails) -> Result<()> {
  // print!("{}", pokemon.ascii_image);

  println!("Digite 1 Para Ver Mais Detalhes");
  println!("Digite 2 Para Escolher {}", pokemon.name);
  // println!("Digite 3 Para Voltar");
  println!("Digite qualquer outro numero pra sair");

  let mut index = String::new();
  let stdin = io::stdin();

  'pokemon_adoption: loop {
    stdin.read_line(&mut index)?;

    let parsed_index: i8 = match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    match parsed_index {
      1 => {
        pokemon_details(pokemon).await?;
        // break 'pokemon_adoption;
      }
      2 => {
        let my = &mut MyPokemon::new(pokemon);
        let _ = my_pokemon(my);
        // break 'pokemon_adoption;
      }
      3 => continue,
      _ => panic!(),
    };
    break 'pokemon_adoption;
  }
  Ok(())
}

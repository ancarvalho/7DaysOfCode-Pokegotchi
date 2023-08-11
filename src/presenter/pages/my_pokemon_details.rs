use crate::domain::{entities::data::Data, enums::screen_possibilities::ScreenPossibilities};
use anyhow::{Ok, Result};
use std::io;

pub fn my_pokemons_details_page(data: &mut Data, index: &usize) -> Result<ScreenPossibilities> {
  // print!("{}", pokemon.ascii_image);


  // let mut mutable_pokemon = my_pokemon.clone();

  let my_pokemon = data.get_pokemon(index.clone());
  println!("Digite 1 Brincar com {}", my_pokemon.pokemon.name);
  println!("Digite 2 Para Alimentar {}", my_pokemon.pokemon.name);
  println!("Digite 3 Para Ver Felicidade {}", my_pokemon.pokemon.name);
  println!("Digite 4 Para Ver Fome {}", my_pokemon.pokemon.name);
  // println!("Digite 5 Para Abandonar {}", my_pokemon.pokemon.name);

  println!("Digite qualquer outro numero pra sair");

  'pokemon_adoption: loop {
    let mut index = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut index)?;

    let parsed_index: i8 = match index.trim().parse::<i8>() {
      core::result::Result::Ok(num) => num,
      core::result::Result::Err(_) => continue,
    };

    match parsed_index {
      1 => {
        my_pokemon.play_with_pokemon().check_pokemon_happiness();
        continue;
      }
      2 => {
        my_pokemon.feed_pokemon().check_pokemon_hungry();
        continue;
      }
      3 => {
        my_pokemon.check_pokemon_happiness();
        continue;
      }
      4 => {
        my_pokemon.check_pokemon_hungry();
        continue;
      }
      // 5 => {
      //   data.remove_pokemon(index);
      //   continue;
      // }


      _ => break 'pokemon_adoption,
    };
  }
  Ok(ScreenPossibilities::Quit)
}

use anyhow::Result;
use std::io;

use crate::domain::enums::screen_possibilities::ScreenPossibilities;

pub fn home_page() -> Result<ScreenPossibilities>  {
  let mut username = String::new();
  let stdin = io::stdin();

  loop {
    println!("Seu Nome: (minimo 2 caracters)");
    stdin.read_line(&mut username)?;

    if username.len() >= 3 {
      println!("{}", username.len());
      break;
    }
  }

  Ok(ScreenPossibilities::PokemonChoosePage { name: username })
  
}

use super::pokemon_details::PokemonDetails;
use anyhow::Result;
use chrono::{prelude::*, Duration};

#[derive(Debug)]
pub struct MyPokemon {
  pub pokemon: PokemonDetails,
  pub last_feed: Option<DateTime<Utc>>,
  pub last_played_with: Option<DateTime<Utc>>,
}

impl MyPokemon {
  pub fn new(pokemon: PokemonDetails) -> MyPokemon {
    MyPokemon {
      pokemon,
      last_feed: None,
      last_played_with: None,
    }
  }

  pub fn feed_pokemon(&mut self) -> &Self {
    println!("Seu Pokemon foi Alimentado");
    // &mut self.last_feed = &mut Some(Utc::now());
    self.last_feed = Some(Utc::now());

    self
  }

  pub fn play_with_pokemon(&mut self) -> &Self {
    println!("Você Brincou Com Seu Pokemon");

    self.last_played_with = Some(Utc::now());

    self

    // Self {
    //   pokemon: self.pokemon.clone(),
    //   last_feed: self.last_feed,
    //   last_played_with: Some(Utc::now()),
    // }
  }

  pub fn check_pokemon_happiness(&self) {
    let last_time = get_duration(&self.last_played_with);
    print!("{}", last_time.num_minutes());
    match last_time.num_minutes() {
      0..=10 => println!("Seu Pokemon Esta Feliz"),
      11..=30 => println!("Seu Pokemon Esta Neutro"),
      31..=60 => println!("Seu Pokemon Esta Entediado"),
      _ => println!("Seu Pokemon Esta Triste"),
    }
  }

  pub fn check_pokemon_hungry(&self) {
    let last_time = get_duration(&self.last_played_with);

    match last_time.num_minutes() {
      1..=10 => println!("Seu Pokemon Esta Alimentado"),
      11..=30 => println!("Seu Pokemon Esta Com Fome"),
      31..=60 => println!("Seu Pokemon Esta Faminto"),
      _ => println!("Seu Pokemon Esta Morto de Fome"),
    }
  }
}

// TODO move to Utils
pub fn get_duration(time: &Option<DateTime<Utc>>) -> Duration {
  match time.clone() {
    Some(time) => Utc::now() - time,
    None => Duration::days(365),
  }
}

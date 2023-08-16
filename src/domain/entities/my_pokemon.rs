use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MyPokemon {
  pub id: i8,
  pub name: String,
  pub pokedex_name: String,
  pub last_feed: Option<String>,
  pub last_played_with: Option<String>,
}
impl MyPokemon {


  pub fn feed_pokemon(&mut self) -> &Self {
    println!("Seu Pokemon foi Alimentado");
    self.last_feed = Some(Utc::now().to_string());

    return self;
  }

  pub fn play_with_pokemon(&mut self) -> &Self {
    println!("Você Brincou Com Seu Pokemon");

    self.last_played_with = Some(Utc::now().to_string());

    return self;
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
    let last_time = get_duration(&self.last_feed);

    match last_time.num_minutes() {
      0..=10 => println!("Seu Pokemon Esta Alimentado"),
      11..=30 => println!("Seu Pokemon Esta Com Fome"),
      31..=60 => println!("Seu Pokemon Esta Faminto"),
      _ => println!("Seu Pokemon Esta Morto de Fome"),
    }
  }
}

// TODO move to Utils
pub fn get_duration(time: &Option<String>) -> Duration {
  // const parsed_item: DateTime<Utc> = DateTime::parse_from_rfc2822(time).unwrap_or(Duration::days(365));
  match time.clone() {
    Some(time) => {
      let now_parsed: DateTime<Utc> = time.parse().unwrap_or(Utc::now());
      now_parsed - Utc::now()
    }
    None => Duration::days(365),
  }
}

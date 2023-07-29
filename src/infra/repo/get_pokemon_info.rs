use crate::domain::entities::{pokemon::Pokemon, pokemon_simplified::PokemonSimplified};
use anyhow::Result;

pub async fn get_pokemon_info(name: String) -> Result<PokemonSimplified> {
  let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
  let resp = reqwest::get(url).await?;
  let simplified_pokemon = resp.json::<Pokemon>().await?.transform();

  Ok(simplified_pokemon)
}

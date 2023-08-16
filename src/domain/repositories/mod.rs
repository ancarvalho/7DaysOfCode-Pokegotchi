use anyhow::Result;
use async_trait::async_trait;

use super::entities::{my_pokemon::MyPokemon, pokemon_details::PokemonDetails};

#[async_trait]
pub trait PokegotchiRepositoryAbstract {
  async fn get_trainer_name(&self) -> Result<String>;
  async fn update_trainer_name(&self, name: String) -> Result<bool>;
  async fn create_trainer(&self, name: String) -> Result<bool>;
  async fn get_pokemons(&self) -> Result<Vec<MyPokemon>>;
  async fn get_pokemon(&self, id: i8) -> Result<MyPokemon>;
  async fn add_pokemon(&self, pokemon: PokemonDetails) -> Result<bool>;
  async fn update_pokemon_played(&self, id: i8) -> Result<bool>;
  async fn update_pokemon_feed(&self, id: i8) -> Result<bool>;
  async fn remove_pokemon(&self, id: i8) -> Result<bool>;
  async fn get_pokemons_count(&self) -> Result<i8>;
}

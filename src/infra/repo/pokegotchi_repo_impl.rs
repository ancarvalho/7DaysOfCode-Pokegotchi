use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;

use crate::domain::{
  entities::{my_pokemon::MyPokemon, pokemon_details::PokemonDetails, trainer::Trainer},
  repositories::PokegotchiRepositoryAbstract, 
};
use anyhow::{Ok, Result};

pub struct PokegotchiRepoImpl {
  pool: SqlitePool,
}

impl PokegotchiRepoImpl {
  pub fn new(sl_pool: SqlitePool) -> Self {
    Self { pool: sl_pool }
  }
}

#[async_trait]
impl PokegotchiRepositoryAbstract for PokegotchiRepoImpl {
  async fn get_pokemons(&self) -> Result<Vec<MyPokemon>> {
    let rec = sqlx::query_as::<_, MyPokemon>(
      "
        SELECT id, name, pokedex_name, last_feed, last_played_with
        FROM pokegotchis
      
    ",
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(rec)
  }

  async fn get_pokemon(&self, id: i8) -> Result<MyPokemon> {
    let rec = sqlx::query_as::<_, MyPokemon>(
      "
        SELECT id, name, pokedex_name, last_feed, last_played_with
        FROM pokegotchis
        WHERE id = $1
      
    ",
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await?;

    Ok(rec)
  }
  async fn add_pokemon(&self, pokemon: PokemonDetails) -> Result<bool> {
    // TODO add later on use-case
    let count = self.get_pokemons_count().await?;
    if count >= 3 {
      return Err(anyhow::Error::msg("Too Many Pokemons"));
    }

    let now: String = Utc::now().to_string();
    let rec = sqlx::query(
      "
        INSERT INTO pokegotchis (name, pokedex_name, last_feed, last_played_with)
        VALUES ($1, $2, $3, $4)
      ",
    )
    .bind(pokemon.name)
    .bind(pokemon.pokedex_name)
    .bind(now.clone())
    .bind(now)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(rec > 0)
  }

  async fn create_trainer(&self, name: String) -> Result<bool> {
    let rec = sqlx::query(
      "
        INSERT INTO trainer (name)
        VALUES ($1)
      ",
    )
    .bind(name)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(rec > 0)
  }

  async fn get_trainer_name(&self) -> Result<String> {
    let rec = sqlx::query_as::<_, Trainer>(
      "
        SELECT *
        FROM trainer
    ",
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(rec.name)
  }
  async fn update_trainer_name(&self, name: String) -> Result<bool> {
    let rec = sqlx::query(
      "
        UPDATE trainer 
        SET name = $1
        
      ",
    )
    .bind(name)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(rec > 0)
  }

  async fn remove_pokemon(&self, id: i8) -> Result<bool> {
    let rec = sqlx::query(
      "
        DELETE FROM pokegotchis
        WHERE  id = $1
      ",
    )
    .bind(id)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(rec > 0)
  }
  async fn get_pokemons_count(&self) -> Result<i8> {
    let rec = sqlx::query_scalar(
      "
        SELECT COUNT(*) as count
        FROM pokegotchis
      
    ",
    )
    .fetch_one(&self.pool)
    .await?;

    Ok(rec)
  }

  async fn update_pokemon_played(&self, id: i8) -> Result<bool> {
    let now = Utc::now().to_string();
    let rec = sqlx::query(
      "
        UPDATE pokegotchis
        SET last_played_with = $2
        WHERE id = $1
      
    ",
    )
    .bind(id)
    .bind(now)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(rec > 0)
  }
  async fn update_pokemon_feed(&self, id: i8) -> Result<bool> {
    let now = Utc::now().to_string();
    let rec = sqlx::query(
      "
        UPDATE pokegotchis
        SET last_feed = $2
        WHERE id = $1
      
    ",
    )
    .bind(id)
    .bind(now)
    .execute(&self.pool)
    .await?
    .rows_affected();

    Ok(rec > 0)
  }
}

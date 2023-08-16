use crate::{
  domain::{entities::pokemon_details::PokemonDetails, repositories::PokegotchiRepositoryAbstract},
  presenter::pages::{home::home_page, pokemon_choose:: pokemon_choose_page, pokemon_adoption::pokemon_adoption_page, pokemon_details::pokemon_details_page,  my_pokemons::my_pokemons_page, my_pokemon_details::my_pokemons_details_page}, infra::repo::pokegotchi_repo_impl::PokegotchiRepoImpl,
};

use anyhow::Result;
use async_recursion::async_recursion;

#[derive(Debug)]
pub enum ScreenPossibilities {
  HomePage ,
  PokemonChoosePage ,
  PokemonAdoptionPage { pokemon: PokemonDetails },
  PokemonDetailsPage { pokemon: PokemonDetails },
  MyPokemonsPage ,
  MyPokomonDetailsPage {index: usize},
  Quit,
}



impl ScreenPossibilities {
  #[async_recursion]
  pub async fn match_possibilities(&self, pokegochi_repo: &PokegotchiRepoImpl) -> Result<ScreenPossibilities>{
    match &self {
      ScreenPossibilities::HomePage  => home_page(pokegochi_repo).await?.match_possibilities(pokegochi_repo).await,
      ScreenPossibilities::PokemonChoosePage => pokemon_choose_page(pokegochi_repo).await?.match_possibilities(pokegochi_repo).await,
      ScreenPossibilities::PokemonAdoptionPage { pokemon } => pokemon_adoption_page(pokemon, pokegochi_repo).await?.match_possibilities(pokegochi_repo).await,
      ScreenPossibilities::PokemonDetailsPage { pokemon } => pokemon_details_page(pokemon, pokegochi_repo).await?.match_possibilities(pokegochi_repo).await,
      ScreenPossibilities::MyPokemonsPage  => my_pokemons_page(pokegochi_repo ).await?.match_possibilities(pokegochi_repo).await,
      ScreenPossibilities::MyPokomonDetailsPage { index }  => my_pokemons_details_page(pokegochi_repo, index).await?.match_possibilities(pokegochi_repo).await,
      ScreenPossibilities::Quit => panic!("User Exited Application"),
    }
  }

  pub async fn start(pokegochi_repo: &PokegotchiRepoImpl) -> Result<ScreenPossibilities> {
    match pokegochi_repo.get_trainer_name().await {
      Ok(_name) => my_pokemons_page(pokegochi_repo).await?.match_possibilities(pokegochi_repo).await,
      Err(_) => ScreenPossibilities::HomePage.match_possibilities(pokegochi_repo).await
    }
    
  }
}

mod config;
mod domain;
mod infra;
mod presenter;

use anyhow::Result;

use crate::{domain::{utils::header::POKEGOTCHI, enums::screen_possibilities::ScreenPossibilities}, infra::{db::{get_db_pool_or_create, migrate_db}, repo::pokegotchi_repo_impl::PokegotchiRepoImpl}};

#[tokio::main]
async fn main() -> Result<()> {
  dotenvy::dotenv().ok();
  let db = get_db_pool_or_create().await?;
  migrate_db(&db).await;

  let pokegotchi_repo = PokegotchiRepoImpl::new(db);

  print!("{}", POKEGOTCHI);
  
  let _ = ScreenPossibilities::start(&pokegotchi_repo).await?;

  Ok(())
}

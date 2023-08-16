use anyhow::{Ok, Result};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub async fn get_db_pool_or_create() -> Result<SqlitePool> {
  let db_url = std::env::var("DATABASE_URL").unwrap();

  if !Sqlite::database_exists(db_url.as_str())
    .await
    .unwrap_or(false)
  {
    println!("Creating database {}", db_url);

    match Sqlite::create_database(db_url.as_str()).await {
      core::result::Result::Ok(_) => println!("Create db success"),
      Err(error) => panic!("error: {}", error),
    }
  } else {
    println!("Database already exists");
  }

  let db = SqlitePool::connect(db_url.as_str()).await.unwrap();

  return Ok(db);
}

pub async fn migrate_db(db: &SqlitePool) {
  let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  let migrations = std::path::Path::new(&crate_dir).join("./migrations");

  println!("{}", &crate_dir);

  let migration_results = sqlx::migrate::Migrator::new(migrations)
    .await
    .unwrap()
    .run(db)
    .await;

  match migration_results {
    core::result::Result::Ok(_) => println!("Migration success"),
    Err(error) => {
      panic!("error: {}", error);
    }
  }
}

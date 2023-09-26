use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Trainer {  
  pub id: i32,
  pub name: String

}
pub mod entities;
pub mod handler;
pub mod router;

use dotenvy::dotenv;
use sea_orm::*;
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
  let db = Database::connect(&db_url).await?;
  Ok(db)
}

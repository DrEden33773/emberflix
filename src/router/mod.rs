pub mod create;
pub mod delete;
pub mod read;
pub mod update;

use rocket::*;

#[get("/")]
pub async fn index() -> &'static str {
  "Hello, emberflix!"
}

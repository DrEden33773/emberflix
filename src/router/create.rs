#![allow(clippy::too_many_arguments)]

use crate::{handler::ErrorResponder, logic::create};
use rocket::*;
use sea_orm::entity::prelude::*;

#[post("/user?<user_type>&<username>&<password>&<display_name>&<birth_date>&<gender>&<phone>")]
pub async fn create_user(
  db: &State<DatabaseConnection>,
  user_type: &str,
  username: &str,
  password: &str,
  display_name: &str,
  birth_date: &str,
  gender: &str,
  phone: &str,
) -> Result<(), ErrorResponder> {
  let db = db as &DatabaseConnection;

  create::create_user(
    db,
    user_type,
    username,
    password,
    display_name,
    birth_date,
    gender,
    phone,
  )
  .await
  .map_err(Into::<ErrorResponder>::into)?;

  Ok(())
}

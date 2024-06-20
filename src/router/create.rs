#![allow(clippy::too_many_arguments)]

use crate::{
  entities::{prelude::*, *},
  handler::ErrorResponder,
};
use rocket::*;
use sea_orm::{entity::prelude::*, *};
use std::str::FromStr;

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

  let new_user = user::ActiveModel {
    user_type: Set(user_type.into()),
    username: Set(username.into()),
    password: Set(password.into()),
    display_name: Set(display_name.into()),
    birth_date: Set(Date::from_str(birth_date).unwrap()),
    gender: Set(gender.into()),
    phone: Set(phone.into()),
    ..Default::default()
  };

  User::insert(new_user)
    .exec(db)
    .await
    .map_err(Into::<ErrorResponder>::into)?;

  Ok(())
}

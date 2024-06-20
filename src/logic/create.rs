use crate::entities::{prelude::*, *};
use sea_orm::*;

pub async fn create_user(
  db: &DatabaseConnection,
  user: user::ActiveModel,
) -> Result<InsertResult<user::ActiveModel>, DbErr> {
  User::insert(user).exec(db).await
}

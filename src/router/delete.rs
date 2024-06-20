use crate::entities::{prelude::*, *};
use sea_orm::{entity::prelude::*, *};

pub async fn delete_user_by_id(db: &DatabaseConnection, id: i64) -> Result<DeleteResult, DbErr> {
  User::delete_by_id(id).exec(db).await
}

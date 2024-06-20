use crate::entities::{prelude::*, *};
use sea_orm::{entity::prelude::*, *};

pub async fn get_all_user(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
  User::find().all(db).await
}

pub async fn get_all_media(db: &DatabaseConnection) -> Result<Vec<media::Model>, DbErr> {
  Media::find().all(db).await
}

pub async fn get_all_tag(db: &DatabaseConnection) -> Result<Vec<tag::Model>, DbErr> {
  Tag::find().all(db).await
}

pub async fn get_all_comment_on_media(
  db: &DatabaseConnection,
) -> Result<Vec<comment_on_media::Model>, DbErr> {
  CommentOnMedia::find().all(db).await
}

pub async fn get_all_comment_on_comment(
  db: &DatabaseConnection,
) -> Result<Vec<comment_on_comment::Model>, DbErr> {
  CommentOnComment::find().all(db).await
}

pub async fn get_ones_follower_names(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<usize>,
) -> Result<Vec<user::Model>, DbErr> {
  todo!()
}

pub async fn get_ones_fans_names(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<usize>,
) -> Result<Vec<user::Model>, DbErr> {
  todo!()
}

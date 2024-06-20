use crate::entities::{prelude::*, *};
use sea_orm::{entity::prelude::*, *};
use subscribe::Relation::User1 as SrcUser;
use subscribe::Relation::User2 as DstUser;

pub async fn get_all_user(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<user::Model>, DbErr> {
  User::find().limit(limit).all(db).await
}

pub async fn get_all_media(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<media::Model>, DbErr> {
  Media::find().limit(limit).all(db).await
}

pub async fn get_all_tag(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<tag::Model>, DbErr> {
  Tag::find().limit(limit).all(db).await
}

pub async fn get_all_comment_on_media(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<comment::Model>, DbErr> {
  Comment::find()
    .filter(comment::Column::MediaId.is_not_null())
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_all_comment_on_comment(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<comment::Model>, DbErr> {
  Comment::find()
    .filter(comment::Column::CommentId.is_not_null())
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_all_comment(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<comment::Model>, DbErr> {
  Comment::find().limit(limit).all(db).await
}

pub async fn get_ones_subscribing_names(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<user::Model>)>, DbErr> {
  User::find_by_id(id)
    .find_with_linked(UserSubscribeUser)
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_ones_subscriber_names(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<user::Model>)>, DbErr> {
  User::find_by_id(id)
    .find_with_linked(UserSubscribedByUser)
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_ones_like_medias(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<media::Model>)>, DbErr> {
  User::find_by_id(id)
    .find_with_linked(UserLikeMedia)
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_media_likers(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(media::Model, Vec<user::Model>)>, DbErr> {
  Media::find_by_id(id)
    .find_with_linked(MediaLikedByUser)
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_ones_favorite_medias(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<media::Model>)>, DbErr> {
  User::find_by_id(id)
    .find_with_linked(UserFavoriteMedia)
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_media_favorite_users(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(media::Model, Vec<user::Model>)>, DbErr> {
  Media::find_by_id(id)
    .find_with_linked(MediaFavoriteByUser)
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_ones_subscribing_users(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<user::Model>)>, DbErr> {
  User::find_by_id(id)
    .find_with_linked(UserSubscribeUser)
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_ones_subscribers(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<user::Model>)>, DbErr> {
  User::find_by_id(id)
    .find_with_linked(UserSubscribedByUser)
    .limit(limit)
    .all(db)
    .await
}

pub struct UserLikeMedia;

impl Linked for UserLikeMedia {
  type FromEntity = user::Entity;
  type ToEntity = media::Entity;

  fn link(&self) -> Vec<sea_orm::LinkDef> {
    vec![
      like::Relation::User.def().rev(),
      like::Relation::Media.def(),
    ]
  }
}

pub struct MediaLikedByUser;

impl Linked for MediaLikedByUser {
  type FromEntity = media::Entity;
  type ToEntity = user::Entity;

  fn link(&self) -> Vec<sea_orm::LinkDef> {
    vec![
      like::Relation::Media.def().rev(),
      like::Relation::User.def(),
    ]
  }
}

pub struct UserFavoriteMedia;

impl Linked for UserFavoriteMedia {
  type FromEntity = user::Entity;
  type ToEntity = media::Entity;

  fn link(&self) -> Vec<sea_orm::LinkDef> {
    vec![
      favorite::Relation::User.def().rev(),
      favorite::Relation::Media.def(),
    ]
  }
}

pub struct MediaFavoriteByUser;

impl Linked for MediaFavoriteByUser {
  type FromEntity = media::Entity;
  type ToEntity = user::Entity;

  fn link(&self) -> Vec<sea_orm::LinkDef> {
    vec![
      favorite::Relation::Media.def().rev(),
      favorite::Relation::User.def(),
    ]
  }
}

pub struct UserSubscribeUser;

impl Linked for UserSubscribeUser {
  type FromEntity = user::Entity;
  type ToEntity = user::Entity;

  fn link(&self) -> Vec<sea_orm::LinkDef> {
    vec![SrcUser.def().rev(), DstUser.def()]
  }
}

pub struct UserSubscribedByUser;

impl Linked for UserSubscribedByUser {
  type FromEntity = user::Entity;
  type ToEntity = user::Entity;

  fn link(&self) -> Vec<sea_orm::LinkDef> {
    vec![DstUser.def().rev(), SrcUser.def()]
  }
}

use crate::entities::{prelude::*, *};
use sea_orm::{entity::prelude::*, *};

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

pub async fn get_ones_subscribing_names(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<user::Model>, DbErr> {
  User::find()
    .select_only()
    .column(user::Column::DisplayName)
    .join(
      JoinType::InnerJoin,
      user::Entity::belongs_to(subscribe::Entity)
        .from(user::Column::Id)
        .to(subscribe::Column::DstId)
        .into(),
    )
    .filter(subscribe::Column::SrcId.eq(id))
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_ones_subscriber_names(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<user::Model>, DbErr> {
  User::find()
    .select_only()
    .column(user::Column::DisplayName)
    .join(
      JoinType::InnerJoin,
      user::Entity::belongs_to(subscribe::Entity)
        .from(user::Column::Id)
        .to(subscribe::Column::SrcId)
        .into(),
    )
    .filter(subscribe::Column::DstId.eq(id))
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_ones_like_medias(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<media::Model>)>, DbErr> {
  todo!()
}

pub async fn get_media_likers(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(media::Model, Vec<user::Model>)>, DbErr> {
  todo!()
}

pub async fn get_ones_subscribing_medias(
  db: &DatabaseConnection,
  id: i64,
  limit: Option<u64>,
) -> Result<Vec<(user::Model, Vec<media::Model>)>, DbErr> {
  todo!()
}

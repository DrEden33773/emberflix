use crate::entities::{prelude::*, *};
use sea_orm::{entity::prelude::*, *};

pub async fn delete_user_by_ids(
  db: &DatabaseConnection,
  ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  User::delete_many()
    .filter(Expr::col(user::Column::Id).is_in(ids))
    .exec(db)
    .await
}

pub async fn delete_media_by_ids(
  db: &DatabaseConnection,
  ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Media::delete_many()
    .filter(Expr::col(media::Column::Id).is_in(ids))
    .exec(db)
    .await
}

pub async fn delete_tag_by_ids(
  db: &DatabaseConnection,
  ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Tag::delete_many()
    .filter(Expr::col(tag::Column::Id).is_in(ids))
    .exec(db)
    .await
}

pub async fn delete_comment_on_media_tree_by_ids(
  db: &DatabaseConnection,
  ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  CommentOnMedia::delete_many()
    .filter(Expr::col(comment_on_media::Column::Id).is_in(ids))
    .exec(db)
    .await
}

pub async fn delete_comment_on_comment_tree_by_ids(
  db: &DatabaseConnection,
  ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  CommentOnComment::delete_many()
    .filter(Expr::col(comment_on_comment::Column::Id).is_in(ids))
    .exec(db)
    .await
}

pub async fn cancel_many_media_tag_by_id(
  db: &DatabaseConnection,
  media_id: i64,
  tag_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  MediaTag::delete_many()
    .filter(Expr::col(media_tag::Column::MediaId).eq(media_id))
    .filter(Expr::col(media_tag::Column::TagId).is_in(tag_ids))
    .exec(db)
    .await
}

pub async fn cancel_many_like_by_id(
  db: &DatabaseConnection,
  user_id: i64,
  media_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Like::delete_many()
    .filter(Expr::col(like::Column::UserId).eq(user_id))
    .filter(Expr::col(like::Column::MediaId).is_in(media_ids))
    .exec(db)
    .await
}

pub async fn cancel_many_favorite_by_id(
  db: &DatabaseConnection,
  user_id: i64,
  media_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Favorite::delete_many()
    .filter(Expr::col(favorite::Column::UserId).eq(user_id))
    .filter(Expr::col(favorite::Column::MediaId).is_in(media_ids))
    .exec(db)
    .await
}

pub async fn cancel_many_subscribe_by_id(
  db: &DatabaseConnection,
  src_id: i64,
  dst_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Subscribe::delete_many()
    .filter(Expr::col(subscribe::Column::SrcId).eq(src_id))
    .filter(Expr::col(subscribe::Column::DstId).is_in(dst_ids))
    .exec(db)
    .await
}

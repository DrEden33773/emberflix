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

pub async fn cancel_media_tag_by_ids(
  db: &DatabaseConnection,
  media_ids: Vec<i64>,
  tag_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  MediaTag::delete_many()
    .filter(Expr::col(media_tag::Column::MediaId).is_in(media_ids))
    .filter(Expr::col(media_tag::Column::TagId).is_in(tag_ids))
    .exec(db)
    .await
}

pub async fn cancel_follow_by_ids(
  db: &DatabaseConnection,
  follower_ids: Vec<i64>,
  followed_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Follow::delete_many()
    .filter(Expr::col(follow::Column::FollowedId).is_in(follower_ids))
    .filter(Expr::col(follow::Column::FollowedId).is_in(followed_ids))
    .exec(db)
    .await
}

pub async fn cancel_like_by_ids(
  db: &DatabaseConnection,
  user_ids: Vec<i64>,
  media_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Like::delete_many()
    .filter(Expr::col(like::Column::UserId).is_in(user_ids))
    .filter(Expr::col(like::Column::MediaId).is_in(media_ids))
    .exec(db)
    .await
}

pub async fn cancel_subscription_by_ids(
  db: &DatabaseConnection,
  user_ids: Vec<i64>,
  media_ids: Vec<i64>,
) -> Result<DeleteResult, DbErr> {
  Subscribe::delete_many()
    .filter(Expr::col(subscribe::Column::UserId).is_in(user_ids))
    .filter(Expr::col(subscribe::Column::MediaId).is_in(media_ids))
    .exec(db)
    .await
}

use crate::entities::{prelude::*, *};
use sea_orm::{entity::prelude::*, entity::*, query::*, *};
use subscribe::Relation::User1 as SrcUser;
use subscribe::Relation::User2 as DstUser;

pub async fn get_many_user(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<user::Model>, DbErr> {
  User::find().limit(limit).all(db).await
}

pub async fn get_many_media(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<media::Model>, DbErr> {
  Media::find().limit(limit).all(db).await
}

pub async fn get_many_tag(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<tag::Model>, DbErr> {
  Tag::find().limit(limit).all(db).await
}

pub async fn get_many_comment_on_media(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<comment::Model>, DbErr> {
  Comment::find()
    .filter(comment::Column::MediaId.is_not_null())
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_many_comment_on_comment(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<comment::Model>, DbErr> {
  Comment::find()
    .filter(comment::Column::CommentId.is_not_null())
    .limit(limit)
    .all(db)
    .await
}

pub async fn get_many_comment(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<comment::Model>, DbErr> {
  Comment::find().limit(limit).all(db).await
}

pub async fn get_comment_tree(_db: &DatabaseConnection, _limit: Option<u64>) {
  println!("\n{}\n", "==========".repeat(3));
  println!("`get_comment_tree` will be implemented ASAP.");
  println!("\n{}\n", "==========".repeat(3));
}

pub async fn get_comment_path_to_comment(
  db: &DatabaseConnection,
  id: i64,
) -> Result<(media::Model, Vec<comment::Model>), DbErr> {
  let mut curr_id: i64 = id;
  let mut comments = vec![];

  while let Some(curr) = Comment::find_by_id(curr_id).one(db).await? {
    comments.push(curr.clone());
    if let Some(father_comment_id) = curr.comment_id {
      curr_id = father_comment_id;
    } else {
      break;
    }
  }

  comments.reverse();
  Ok((Media::find_by_id(curr_id).one(db).await?.unwrap(), comments))
}

pub async fn get_comment_path_to_comment_with_recursive(
  db: &DatabaseConnection,
  id: i64,
) -> Result<(media::Model, Vec<comment::Model>), DbErr> {
  // raw sql
  let raw_sql = format!(
    "
    WITH RECURSIVE comment_chain AS (
      SELECT * FROM comments WHERE id = {id}
      UNION ALL
      SELECT comments.* FROM comments
      JOIN comment_chain ON comments.id = comment_chain.comment_id
    )
    SELECT * FROM comment_chain;
    "
  );

  let comments: Vec<comment::Model> = comment::Entity::find()
    .from_raw_sql(Statement::from_sql_and_values(
      db.get_database_backend(),
      raw_sql,
      [],
    ))
    .all(db)
    .await?;

  // Assume `comments` is reversed compares to the root-to-leaf path
  // (It depends on the implementation of db)
  // If the assumption works, then the last (root element)
  // should have non_nullable `comment_id` as `media_id`
  let media_id = comments.last().unwrap().comment_id.unwrap();
  let media = Media::find_by_id(media_id).one(db).await?.unwrap();

  Ok((media, comments))
}

pub async fn get_many_media_tag(
  db: &DatabaseConnection,
  limit: Option<u64>,
) -> Result<Vec<media_tag::Model>, DbErr> {
  MediaTag::find().limit(limit).all(db).await
}

pub async fn get_user_subscribing(
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

pub async fn get_user_subscribers(
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

pub async fn get_user_subscribing_users(
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

pub async fn get_user_like_medias(
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

pub async fn get_user_favorite_medias(
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

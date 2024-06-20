#![allow(clippy::too_many_arguments)]

use crate::entities::{prelude::*, *};
use sea_orm::{entity::prelude::*, *};
use sea_query::OnConflict;
use std::str::FromStr;

pub async fn create_user(
  db: &DatabaseConnection,
  user_type: &str,
  username: &str,
  password: &str,
  display_name: &str,
  birth_date: &str,
  gender: &str,
  phone: &str,
) -> Result<InsertResult<user::ActiveModel>, DbErr> {
  let model = user::ActiveModel {
    user_type: Set(user_type.into()),
    username: Set(username.into()),
    password: Set(password.into()),
    display_name: Set(display_name.into()),
    birth_date: Set(Date::from_str(birth_date).unwrap()),
    gender: Set(gender.into()),
    phone: Set(phone.into()),
    ..Default::default()
  };

  User::insert(model).exec(db).await
}

pub async fn create_tag(
  db: &DatabaseConnection,
  name: &str,
) -> Result<InsertResult<tag::ActiveModel>, DbErr> {
  let model = tag::ActiveModel {
    name: Set(name.into()),
    ..Default::default()
  };

  Tag::insert(model)
    .on_conflict(
      OnConflict::column(tag::Column::Name)
        .do_nothing()
        .to_owned(),
    )
    .exec(db)
    .await
}

pub async fn tag_media_with(
  db: &DatabaseConnection,
  media_id: i64,
  tag_id: i64,
) -> Result<InsertResult<media_tag::ActiveModel>, DbErr> {
  let model = media_tag::ActiveModel {
    media_id: Set(media_id),
    tag_id: Set(tag_id),
  };

  MediaTag::insert(model).exec(db).await
}

pub async fn create_media(
  db: &DatabaseConnection,
  media_type: &str,
  title: &str,
  brief: &str,
  content_url: &str,
  uploader_id: i64,
  tags: Vec<&str>,
) -> Result<InsertResult<media::ActiveModel>, DbErr> {
  let model = media::ActiveModel {
    media_type: Set(media_type.into()),
    title: Set(title.into()),
    brief: Set(brief.into()),
    content_url: Set(content_url.into()),
    uploader_id: Set(uploader_id),
    review_passed: Set(false),
    ..Default::default()
  };

  let created_media = Media::insert(model).exec(db).await?;
  let new_media_id = created_media.last_insert_id;

  for tag in tags {
    let created_tag = create_tag(db, tag).await?;
    let new_tag_id = created_tag.last_insert_id;
    tag_media_with(db, new_media_id, new_tag_id).await?;
  }

  Ok(created_media)
}

pub async fn create_comment_on_media(
  db: &DatabaseConnection,
  commenter_id: i64,
  media_id: i64,
  content: &str,
) -> Result<InsertResult<comment_on_media::ActiveModel>, DbErr> {
  let model = comment_on_media::ActiveModel {
    commenter_id: Set(commenter_id),
    media_id: Set(media_id),
    content: Set(content.into()),
    review_passed: Set(false),
    ..Default::default()
  };

  CommentOnMedia::insert(model).exec(db).await
}

pub async fn create_comment_on_comment(
  db: &DatabaseConnection,
  commenter_id: i64,
  commented_id: i64,
  content: &str,
) -> Result<InsertResult<comment_on_comment::ActiveModel>, DbErr> {
  let model = comment_on_comment::ActiveModel {
    commenter_id: Set(commenter_id),
    commented_id: Set(commented_id),
    content: Set(content.into()),
    review_passed: Set(false),
    ..Default::default()
  };

  CommentOnComment::insert(model).exec(db).await
}

pub async fn user_like_media(
  db: &DatabaseConnection,
  user_id: i64,
  media_id: i64,
) -> Result<InsertResult<like::ActiveModel>, DbErr> {
  let model = like::ActiveModel {
    user_id: Set(user_id),
    media_id: Set(media_id),
  };

  Like::insert(model).exec(db).await
}

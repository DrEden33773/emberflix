//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub media_type: String,
  pub title: String,
  #[sea_orm(column_type = "Text")]
  pub brief: String,
  pub content_url: String,
  pub uploader_id: i64,
  pub review_passed: bool,
  pub requested_at: DateTime,
  pub published_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::comment_on_comment::Entity")]
  CommentOnComment,
  #[sea_orm(has_many = "super::comment_on_media::Entity")]
  CommentOnMedia,
  #[sea_orm(has_many = "super::media_tag::Entity")]
  MediaTag,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UploaderId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  User,
}

impl Related<super::comment_on_comment::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::CommentOnComment.def()
  }
}

impl Related<super::comment_on_media::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::CommentOnMedia.def()
  }
}

impl Related<super::media_tag::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::MediaTag.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl Related<super::tag::Entity> for Entity {
  fn to() -> RelationDef {
    super::media_tag::Relation::Tag.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::media_tag::Relation::Media.def().rev())
  }
}

impl ActiveModelBehavior for ActiveModel {}

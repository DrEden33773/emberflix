//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub user_type: String,
  #[sea_orm(unique)]
  pub username: String,
  pub password: String,
  pub display_name: String,
  pub registry_date: Date,
  pub birth_date: Date,
  pub gender: String,
  pub phone: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::comment::Entity")]
  Comment,
  #[sea_orm(has_many = "super::favorite::Entity")]
  Favorite,
  #[sea_orm(has_many = "super::like::Entity")]
  Like,
  #[sea_orm(has_many = "super::media::Entity")]
  Media,
}

impl Related<super::comment::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Comment.def()
  }
}

impl Related<super::favorite::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Favorite.def()
  }
}

impl Related<super::like::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Like.def()
  }
}

impl Related<super::media::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Media.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

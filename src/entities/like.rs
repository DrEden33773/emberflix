//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "like")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub user_id: i64,
  #[sea_orm(primary_key, auto_increment = false)]
  pub media_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::media::Entity",
    from = "Column::MediaId",
    to = "super::media::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Media,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  User,
}

impl Related<super::media::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Media.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

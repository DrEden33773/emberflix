use sea_orm_migration::prelude::*;

use crate::m20240613_000001_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub(crate) enum Follow {
  Table,
  SrcId,
  DstId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Follow::Table)
          .if_not_exists()
          .col(ColumnDef::new(Follow::SrcId).big_integer().not_null())
          .col(ColumnDef::new(Follow::DstId).big_integer().not_null())
          .primary_key(
            Index::create()
              .name("pk-user_follow_user")
              .col(Follow::SrcId)
              .col(Follow::DstId)
              .primary(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-follow-src_id")
              .from(Follow::Table, Follow::SrcId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-follow-dst_id")
              .from(Follow::Table, Follow::DstId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(Follow::Table).to_owned())
      .await
  }
}

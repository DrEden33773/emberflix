use sea_orm_migration::prelude::*;

use crate::m20240613_000001_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Subscribe {
  Table,
  UserId,
  MediaId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Subscribe::Table)
          .if_not_exists()
          .col(ColumnDef::new(Subscribe::UserId).big_integer().not_null())
          .col(ColumnDef::new(Subscribe::MediaId).big_integer().not_null())
          .primary_key(
            Index::create()
              .name("pk-user_subscribe_media")
              .col(Subscribe::UserId)
              .col(Subscribe::MediaId)
              .primary(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-subscribe-user_id")
              .from(Subscribe::Table, Subscribe::UserId)
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
      .drop_table(Table::drop().if_exists().table(Subscribe::Table).to_owned())
      .await
  }
}

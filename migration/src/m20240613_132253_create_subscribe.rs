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
              .on_delete(ForeignKeyAction::Restrict)
              .on_update(ForeignKeyAction::Restrict),
          )
          .to_owned(),
      )
      .await?;

    let user_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-subscribe-user_id")
      .table(Subscribe::Table)
      .col(Subscribe::UserId)
      .to_owned();
    let media_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-subscribe-media_id")
      .table(Subscribe::Table)
      .col(Subscribe::MediaId)
      .to_owned();

    manager
      .get_connection()
      .get_database_backend()
      .build(&user_id_idx);
    manager
      .get_connection()
      .get_database_backend()
      .build(&media_id_idx);

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(Subscribe::Table).to_owned())
      .await
  }
}

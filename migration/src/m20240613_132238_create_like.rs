use sea_orm_migration::prelude::*;

use crate::m20240613_000001_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Like {
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
          .table(Like::Table)
          .if_not_exists()
          .col(ColumnDef::new(Like::UserId).big_integer().not_null())
          .col(ColumnDef::new(Like::MediaId).big_integer().not_null())
          .primary_key(
            Index::create()
              .name("pk-user_like_media")
              .col(Like::UserId)
              .col(Like::MediaId)
              .primary(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-like-user_id")
              .from(Like::Table, Like::UserId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Restrict)
              .on_update(ForeignKeyAction::Restrict),
          )
          .to_owned(),
      )
      .await?;

    let user_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-like-user_id")
      .table(Like::Table)
      .col(Like::UserId)
      .to_owned();
    let media_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-like-media_id")
      .table(Like::Table)
      .col(Like::MediaId)
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
      .drop_table(Table::drop().if_exists().table(Like::Table).to_owned())
      .await
  }
}

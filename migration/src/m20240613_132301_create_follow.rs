use sea_orm_migration::prelude::*;

use crate::m20240613_000001_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub(crate) enum Follow {
  Table,
  FollowerId,
  FollowedId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Follow::Table)
          .if_not_exists()
          .col(ColumnDef::new(Follow::FollowerId).big_integer().not_null())
          .col(ColumnDef::new(Follow::FollowedId).big_integer().not_null())
          .primary_key(
            Index::create()
              .name("pk-user_follow_user")
              .col(Follow::FollowerId)
              .col(Follow::FollowedId)
              .primary(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-follow-follower_id")
              .from(Follow::Table, Follow::FollowerId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Restrict)
              .on_update(ForeignKeyAction::Restrict),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-follow-followed_id")
              .from(Follow::Table, Follow::FollowedId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Restrict)
              .on_update(ForeignKeyAction::Restrict),
          )
          .to_owned(),
      )
      .await?;

    let follower_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-follow-follower_id")
      .table(Follow::Table)
      .col(Follow::FollowerId)
      .to_owned();
    let followed_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-follow-followed_id")
      .table(Follow::Table)
      .col(Follow::FollowedId)
      .to_owned();

    manager
      .get_connection()
      .get_database_backend()
      .build(&follower_id_idx);
    manager
      .get_connection()
      .get_database_backend()
      .build(&followed_id_idx);

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(Follow::Table).to_owned())
      .await
  }
}

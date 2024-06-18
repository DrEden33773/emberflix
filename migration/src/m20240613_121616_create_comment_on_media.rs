use sea_orm_migration::prelude::*;

use crate::{m20240613_000001_create_user::User, m20240613_120211_create_media::Media};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum CommentOnMedia {
  Table,
  Id,
  CommenterID,
  MediaID,
  Content,
  ReviewPassed,
  RequestedAt,
  PublishedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(CommentOnMedia::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(CommentOnMedia::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(
            ColumnDef::new(CommentOnMedia::CommenterID)
              .big_integer()
              .not_null(),
          )
          .col(
            ColumnDef::new(CommentOnMedia::MediaID)
              .big_integer()
              .not_null(),
          )
          .col(ColumnDef::new(CommentOnMedia::Content).text().not_null())
          .col(
            ColumnDef::new(CommentOnMedia::ReviewPassed)
              .boolean()
              .default(false)
              .not_null(),
          )
          .col(
            ColumnDef::new(CommentOnMedia::RequestedAt)
              .timestamp()
              .default(Expr::current_timestamp())
              .not_null(),
          )
          .col(
            ColumnDef::new(CommentOnMedia::PublishedAt)
              .timestamp()
              .null(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment_on_media-commenter_id")
              .from(CommentOnMedia::Table, CommentOnMedia::CommenterID)
              .to(User::Table, User::Id),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment_on_media-media_id")
              .from(CommentOnMedia::Table, CommentOnMedia::MediaID)
              .to(Media::Table, Media::Id)
              .on_delete(ForeignKeyAction::Restrict)
              .on_update(ForeignKeyAction::Restrict),
          )
          .to_owned(),
      )
      .await?;

    let idx = Index::create()
      .index_type(IndexType::FullText)
      .name("idx-comment_on_media-content")
      .table(CommentOnMedia::Table)
      .col(CommentOnMedia::Content)
      .to_owned();

    manager.get_connection().get_database_backend().build(&idx);

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(
        Table::drop()
          .if_exists()
          .table(CommentOnMedia::Table)
          .to_owned(),
      )
      .await
  }
}

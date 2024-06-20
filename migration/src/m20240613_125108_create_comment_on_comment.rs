use sea_orm_migration::prelude::*;

use crate::{m20240613_000001_create_user::User, m20240613_120211_create_media::Media};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum CommentOnComment {
  Table,
  Id,
  CommenterID,
  CommentedID,
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
          .table(CommentOnComment::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(CommentOnComment::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(
            ColumnDef::new(CommentOnComment::CommenterID)
              .big_integer()
              .not_null(),
          )
          .col(
            ColumnDef::new(CommentOnComment::CommentedID)
              .big_integer()
              .not_null(),
          )
          .col(ColumnDef::new(CommentOnComment::Content).text().not_null())
          .col(
            ColumnDef::new(CommentOnComment::ReviewPassed)
              .boolean()
              .default(false)
              .not_null(),
          )
          .col(
            ColumnDef::new(CommentOnComment::RequestedAt)
              .date_time()
              .default(Expr::current_timestamp())
              .not_null(),
          )
          .col(
            ColumnDef::new(CommentOnComment::PublishedAt)
              .date_time()
              .null(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment_on_comment-commenter_id")
              .from(CommentOnComment::Table, CommentOnComment::CommenterID)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment_on_comment-commented_id")
              .from(CommentOnComment::Table, CommentOnComment::CommentedID)
              .to(Media::Table, Media::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .if_not_exists()
          .name("idx-comment_on_comment-published_at")
          .index_type(IndexType::BTree)
          .table(CommentOnComment::Table)
          .col(CommentOnComment::PublishedAt)
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(
        Table::drop()
          .if_exists()
          .table(CommentOnComment::Table)
          .to_owned(),
      )
      .await
  }
}

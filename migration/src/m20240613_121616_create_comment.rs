use sea_orm_migration::prelude::*;

use crate::{m20240613_000001_create_user::User, m20240613_120211_create_media::Media};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Comment {
  Table,
  Id,
  UserId,
  MediaId,
  CommentId,
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
          .table(Comment::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Comment::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(Comment::UserId).big_integer().not_null())
          .col(ColumnDef::new(Comment::MediaId).big_integer().null())
          .col(ColumnDef::new(Comment::CommentId).big_integer().null())
          .col(ColumnDef::new(Comment::Content).text().not_null())
          .col(
            ColumnDef::new(Comment::ReviewPassed)
              .boolean()
              .default(false)
              .not_null(),
          )
          .col(
            ColumnDef::new(Comment::RequestedAt)
              .date_time()
              .default(Expr::current_timestamp())
              .not_null(),
          )
          .col(ColumnDef::new(Comment::PublishedAt).date_time().null())
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment-user_id")
              .from(Comment::Table, Comment::UserId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment-media_id")
              .from(Comment::Table, Comment::MediaId)
              .to(Media::Table, Media::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment-comment_id")
              .from(Comment::Table, Comment::CommentId)
              .to(Comment::Table, Comment::Id)
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
          .name("idx-comment_on_media-published_at")
          .index_type(IndexType::BTree)
          .table(Comment::Table)
          .col(Comment::PublishedAt)
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(Comment::Table).to_owned())
      .await
  }
}

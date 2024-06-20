use crate::m20240613_000001_create_user::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub(crate) enum Media {
  Table,
  Id,
  MediaType,
  Title,
  Brief,
  ContentURL,
  UploaderID,
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
          .table(Media::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Media::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(
            ColumnDef::new(Media::MediaType)
              .string()
              .check(Expr::col(Media::MediaType).is_in(["video", "music", "article"]))
              .not_null(),
          )
          .col(ColumnDef::new(Media::Title).string().not_null())
          .col(
            ColumnDef::new(Media::Brief)
              .text()
              .default("作者有点懒, 所以没有简介...")
              .not_null(),
          )
          .col(ColumnDef::new(Media::ContentURL).string().not_null())
          .col(ColumnDef::new(Media::UploaderID).big_integer().not_null())
          .col(
            ColumnDef::new(Media::ReviewPassed)
              .boolean()
              .default(false)
              .not_null(),
          )
          .col(
            ColumnDef::new(Media::RequestedAt)
              .date_time()
              .default(Expr::current_timestamp())
              .not_null(),
          )
          .col(ColumnDef::new(Media::PublishedAt).date_time().null())
          .foreign_key(
            ForeignKey::create()
              .name("fk-media-uploader_id")
              .from(Media::Table, Media::UploaderID)
              .to(User::Table, User::Id),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .if_not_exists()
          .name("idx-media-title")
          .index_type(IndexType::BTree)
          .table(Media::Table)
          .col(Media::Title)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .if_not_exists()
          .name("idx-media-published_at")
          .index_type(IndexType::BTree)
          .table(Media::Table)
          .col(Media::PublishedAt)
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(Media::Table).to_owned())
      .await
  }
}

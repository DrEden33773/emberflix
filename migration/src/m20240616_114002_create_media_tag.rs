use sea_orm_migration::prelude::*;

use crate::{m20240613_120211_create_media::Media, m20240613_132154_create_tag::Tag};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum MediaTag {
  Table,
  MediaId,
  TagId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(MediaTag::Table)
          .if_not_exists()
          .col(ColumnDef::new(MediaTag::MediaId).big_integer().not_null())
          .col(ColumnDef::new(MediaTag::TagId).big_integer().not_null())
          .primary_key(
            Index::create()
              .name("pk-media_tag")
              .col(MediaTag::MediaId)
              .col(MediaTag::TagId)
              .primary(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-media_tag-media_id")
              .from(MediaTag::Table, MediaTag::MediaId)
              .to(Media::Table, Media::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-media_tag-tag_id")
              .from(MediaTag::Table, MediaTag::TagId)
              .to(Tag::Table, Tag::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(MediaTag::Table).to_owned())
      .await
  }
}

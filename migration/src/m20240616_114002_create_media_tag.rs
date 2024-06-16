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
              .on_delete(ForeignKeyAction::Restrict)
              .on_update(ForeignKeyAction::Restrict),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-media_tag-tag_id")
              .from(MediaTag::Table, MediaTag::TagId)
              .to(Tag::Table, Tag::Id)
              .on_delete(ForeignKeyAction::Restrict)
              .on_update(ForeignKeyAction::Restrict),
          )
          .to_owned(),
      )
      .await?;

    let media_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-media_tag-media_id")
      .table(MediaTag::Table)
      .col(MediaTag::MediaId)
      .to_owned();
    let tag_id_idx = Index::create()
      .index_type(IndexType::BTree)
      .name("idx-media_tag-tag_id")
      .table(MediaTag::Table)
      .col(MediaTag::TagId)
      .to_owned();

    manager
      .get_connection()
      .get_database_backend()
      .build(&media_id_idx);
    manager
      .get_connection()
      .get_database_backend()
      .build(&tag_id_idx);

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(MediaTag::Table).to_owned())
      .await
  }
}

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub(crate) enum Tag {
  Table,
  Id,
  Name,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Tag::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Tag::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(Tag::Name).string().unique_key().not_null())
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .if_not_exists()
          .index_type(IndexType::BTree)
          .name("idx-tag-name")
          .table(Tag::Table)
          .col(Tag::Name)
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(Tag::Table).to_owned())
      .await
  }
}

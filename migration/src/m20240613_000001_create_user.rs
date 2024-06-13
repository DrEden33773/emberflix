use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub(crate) enum User {
  Table,
  Id,
  UserType,
  Username,
  Password,
  DisplayName,
  RegistryDate,
  BirthDate,
  Gender,
  Phone,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(User::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(
            ColumnDef::new(User::UserType)
              .string()
              .default("User")
              .check(Expr::col(User::UserType).is_in(["admin", "user"]))
              .not_null(),
          )
          .col(
            ColumnDef::new(User::Username)
              .string()
              .unique_key()
              .not_null(),
          )
          .col(ColumnDef::new(User::Password).string().not_null())
          .col(ColumnDef::new(User::DisplayName).string().not_null())
          .col(
            ColumnDef::new(User::RegistryDate)
              .date()
              .default(Expr::current_date())
              .not_null(),
          )
          .col(ColumnDef::new(User::BirthDate).date().not_null())
          .col(
            ColumnDef::new(User::Gender)
              .string()
              .check(Expr::col(User::Gender).is_in(["Male", "Female", "Other"]))
              .not_null(),
          )
          .col(ColumnDef::new(User::Phone).string().not_null())
          .to_owned(),
      )
      .await?;

    Index::create()
      .name("idx-user-user_name")
      .index_type(IndexType::BTree)
      .table(User::Table)
      .col(User::Username);

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
      .await
  }
}

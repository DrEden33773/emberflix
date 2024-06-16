pub use sea_orm_migration::prelude::*;

mod m20240613_000001_create_user;
mod m20240613_120211_create_media;
mod m20240613_121616_create_comment_on_media;
mod m20240613_125108_create_comment_on_comment;
mod m20240613_132154_create_tag;
mod m20240613_132238_create_like;
mod m20240613_132253_create_subscribe;
mod m20240613_132301_create_follow;
mod m20240616_114002_create_media_tag;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20240613_000001_create_user::Migration),
            Box::new(m20240613_120211_create_media::Migration),
            Box::new(m20240613_121616_create_comment_on_media::Migration),
            Box::new(m20240613_125108_create_comment_on_comment::Migration),
            Box::new(m20240613_132154_create_tag::Migration),
            Box::new(m20240613_132238_create_like::Migration),
            Box::new(m20240613_132253_create_subscribe::Migration),
            Box::new(m20240613_132301_create_follow::Migration),
            Box::new(m20240616_114002_create_media_tag::Migration),
        ]
  }
}

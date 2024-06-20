#![allow(clippy::too_many_arguments)]

use crate::entities::*;
use chrono::Utc;
use sea_orm::{entity::prelude::*, *};
use std::str::FromStr;

pub struct MediaUpdateBuilder<'a> {
  id: i64,
  db: &'a DatabaseConnection,
  media_type: Option<&'a str>,
  title: Option<&'a str>,
  brief: Option<&'a str>,
  content_url: Option<&'a str>,
}

impl<'a> MediaUpdateBuilder<'a> {
  pub async fn update(&self) -> Result<media::Model, DbErr> {
    let model = media::ActiveModel {
      id: Set(self.id),
      media_type: match self.media_type {
        Some(media_type) => Set(media_type.into()),
        None => NotSet,
      },
      title: match self.title {
        Some(title) => Set(title.into()),
        None => NotSet,
      },
      brief: match self.brief {
        Some(brief) => Set(brief.into()),
        None => NotSet,
      },
      content_url: match self.content_url {
        Some(content_url) => Set(content_url.into()),
        None => NotSet,
      },
      uploader_id: NotSet,
      review_passed: NotSet,
      requested_at: Set(Utc::now().naive_utc()),
      published_at: NotSet,
    };

    model.update(self.db).await
  }
}

impl<'a> MediaUpdateBuilder<'a> {
  pub fn new(id: i64, db: &'a DatabaseConnection) -> Self {
    Self {
      id,
      db,
      media_type: None,
      title: None,
      brief: None,
      content_url: None,
    }
  }
  pub fn media_type(&mut self, media_type: &'a str) {
    self.media_type = media_type.into();
  }
  pub fn title(&mut self, title: &'a str) {
    self.title = title.into();
  }
  pub fn brief(&mut self, brief: &'a str) {
    self.brief = brief.into();
  }
  pub fn content_url(&mut self, content_url: &'a str) {
    self.content_url = content_url.into();
  }
}

pub struct UserUpdateBuilder<'a> {
  id: i64,
  db: &'a DatabaseConnection,
  user_type: Option<&'a str>,
  username: Option<&'a str>,
  password: Option<&'a str>,
  display_name: Option<&'a str>,
  birth_date: Option<&'a str>,
  gender: Option<&'a str>,
  phone: Option<&'a str>,
}

impl<'a> UserUpdateBuilder<'a> {
  pub async fn update(&self) -> Result<user::Model, DbErr> {
    let model = user::ActiveModel {
      id: Set(self.id),
      user_type: match self.user_type {
        Some(user_type) => Set(user_type.into()),
        None => NotSet,
      },
      username: match self.username {
        Some(username) => Set(username.into()),
        None => NotSet,
      },
      password: match self.password {
        Some(password) => Set(password.into()),
        None => NotSet,
      },
      display_name: match self.display_name {
        Some(display_name) => Set(display_name.into()),
        None => NotSet,
      },
      birth_date: match self.birth_date {
        Some(birth_date) => Set(Date::from_str(birth_date).unwrap()),
        None => NotSet,
      },
      gender: match self.gender {
        Some(gender) => Set(gender.into()),
        None => NotSet,
      },
      phone: match self.phone {
        Some(phone) => Set(phone.into()),
        None => NotSet,
      },
      registry_date: NotSet,
    };

    model.update(self.db).await
  }
}

impl<'a> UserUpdateBuilder<'a> {
  pub fn new(id: i64, db: &'a DatabaseConnection) -> Self {
    Self {
      id,
      db,
      user_type: None,
      username: None,
      password: None,
      display_name: None,
      birth_date: None,
      gender: None,
      phone: None,
    }
  }
  pub fn user_type(&mut self, user_type: &'a str) {
    self.user_type = user_type.into();
  }
  pub fn username(&mut self, username: &'a str) {
    self.username = username.into();
  }
  pub fn password(&mut self, password: &'a str) {
    self.password = password.into();
  }
  pub fn display_name(&mut self, display_name: &'a str) {
    self.display_name = display_name.into();
  }
  pub fn birth_date(&mut self, birth_date: &'a str) {
    self.birth_date = birth_date.into();
  }
  pub fn gender(&mut self, gender: &'a str) {
    self.gender = gender.into();
  }
  pub fn phone(&mut self, phone: &'a str) {
    self.phone = phone.into();
  }
}

pub async fn pass_media_review(db: &DatabaseConnection, id: i64) -> Result<media::Model, DbErr> {
  let model = media::ActiveModel {
    id: Set(id),
    review_passed: Set(true),
    ..Default::default()
  };

  model.update(db).await
}

pub async fn pass_comment_on_media_review(
  db: &DatabaseConnection,
  id: i64,
) -> Result<comment_on_media::Model, DbErr> {
  let model = comment_on_media::ActiveModel {
    id: Set(id),
    review_passed: Set(true),
    ..Default::default()
  };

  model.update(db).await
}

pub async fn pass_comment_on_comment_review(
  db: &DatabaseConnection,
  id: i64,
) -> Result<comment_on_comment::Model, DbErr> {
  let model = comment_on_comment::ActiveModel {
    id: Set(id),
    review_passed: Set(true),
    ..Default::default()
  };

  model.update(db).await
}

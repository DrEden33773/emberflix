# Copilot QA

对于下面的定义:

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub user_type: String,
  #[sea_orm(unique)]
  pub username: String,
  pub password: String,
  pub display_name: String,
  pub registry_date: Date,
  pub birth_date: Date,
  pub gender: String,
  pub phone: String,
}
```

有对应的 `create` 实现:

```rust
pub async fn create_user(
  db: &DatabaseConnection,
  user_type: &str,
  username: &str,
  password: &str,
  display_name: &str,
  birth_date: &str,
  gender: &str,
  phone: &str,
) -> Result<(), ErrorResponder> {
  let new_user = user::ActiveModel {
    user_type: Set(user_type.into()),
    username: Set(username.into()),
    password: Set(password.into()),
    display_name: Set(display_name.into()),
    birth_date: Set(Date::from_str(birth_date).unwrap()),
    gender: Set(gender.into()),
    phone: Set(phone.into()),
    ..Default::default()
  };

  User::insert(new_user)
    .exec(db)
    .await
    .map_err(Into::<ErrorResponder>::into)?;

  Ok(())
}
```

请你仿照这样的关系, 为下面的定义, 进行 `create` 实现:

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub media_type: String,
  pub title: String,
  #[sea_orm(column_type = "Text")]
  pub brief: String,
  pub content_url: String,
  pub uploader_id: i64,
  pub review_passed: bool,
  pub requested_at: DateTime,
  pub published_at: Option<DateTime>,
}
```

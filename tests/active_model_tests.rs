//! Run with
//!
//! ```not_rust
//! cargo test --features=sea-orm/sqlx-sqlite,sea-orm/runtime-tokio --test active_model_tests
//! ```

pub mod common;

use common::dto::user::*;
use sea_orm::*;
use sea_orm_crud::*;

#[tokio::test]
async fn main() -> Result<(), anyhow::Error> {
  assert_eq!(
    CreateUser {
      username: "ahas".to_owned(),
      age: Nullable::Null,
      role: "admin".to_owned(),
      is_active: Optional(None),
    }
    .into_active_model(),
    common::schema::users::ActiveModel {
      username: Set("ahas".to_owned()),
      age: Set(None),
      role: Set("admin".to_owned()),
      is_active: NotSet,
      ..Default::default()
    }
  );

  assert_eq!(
    CreateUser {
      username: "ahas".to_owned(),
      age: Nullable::Value(17),
      role: "admin".to_owned(),
      is_active: Optional(Some(true)),
    }
    .into_active_model(),
    common::schema::users::ActiveModel {
      username: Set("ahas".to_owned()),
      age: Set(Some(17)),
      role: Set("admin".to_owned()),
      is_active: Set(true),
      ..Default::default()
    }
  );

  assert_eq!(
    CreateUser {
      username: "ahas".to_owned(),
      age: Nullable::Undefined,
      role: "admin".to_owned(),
      is_active: Optional(None),
    }
    .into_active_model(),
    common::schema::users::ActiveModel {
      username: Set("ahas".to_owned()),
      age: NotSet,
      role: Set("admin".to_owned()),
      is_active: NotSet,
      ..Default::default()
    }
  );

  Ok(())
}

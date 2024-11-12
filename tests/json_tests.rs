//! Run with
//!
//! ```not_rust
//! cargo test --features=sea-orm/sqlx-sqlite,sea-orm/runtime-tokio --test json_tests
//! ```

pub mod common;

use common::dto::user::*;
use sea_orm::*;
use sea_orm_crud::*;
use serde_json::json;

#[tokio::test]
async fn main() -> Result<(), anyhow::Error> {
  let dto = json_to_dto(json!({
    "username": "ahas",
    "age": null,
    "role": "admin",
  }))?;

  assert_eq!(
    dto,
    CreateUser {
      username: "ahas".to_owned(),
      age: Nullable::Null,
      role: "admin".to_owned(),
      is_active: Optional(None),
    }
  );
  assert_eq!(
    dto.into_active_model(),
    common::schema::users::ActiveModel {
      username: Set("ahas".to_owned()),
      age: Set(None),
      role: Set("admin".to_owned()),
      is_active: NotSet,
      ..Default::default()
    }
  );

  let dto = json_to_dto(json!({
    "username": "ahas",
    "age": 17,
    "role": "admin",
    "isActive": true
  }))?;
  assert_eq!(
    dto,
    CreateUser {
      username: "ahas".to_owned(),
      age: Nullable::Value(17),
      role: "admin".to_owned(),
      is_active: Optional(Some(true))
    }
  );
  assert_eq!(
    dto.into_active_model(),
    common::schema::users::ActiveModel {
      username: Set("ahas".to_owned()),
      age: Set(Some(17)),
      role: Set("admin".to_owned()),
      is_active: Set(true),
      ..Default::default()
    }
  );

  let dto = json_to_dto(json!({
    "username": "ahas",
    "role": "admin",
  }))?;
  assert_eq!(
    dto,
    CreateUser {
      username: "ahas".to_owned(),
      role: "admin".to_owned(),
      age: Nullable::Undefined,
      is_active: Optional(None),
    }
  );
  assert_eq!(
    dto.into_active_model(),
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

fn json_to_dto(value: serde_json::Value) -> Result<CreateUser, anyhow::Error> {
  let dto: CreateUser = serde_json::from_value(value)?;

  Ok(dto)
}

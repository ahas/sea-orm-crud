#![allow(unused_imports, dead_code)]

pub mod common;

use common::*;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{DbErr, EntityTrait, IntoActiveModel, Set};
use sea_orm_crud::*;

use self::dto::user::GetUser;
use self::schema::users::ActiveModel;

#[tokio::test]
async fn main() -> Result<(), DbErr> {
  let ctx = TestContext::new("crud_tests").await;

  schema::create_tables(&ctx.db_conn).await?;

  let new_user = create_user_active_model();
  schema::users::Entity::insert(new_user).exec(&ctx.db_conn).await?;

  let user = schema::users::Entity::find_by_id("ahas")
    .into_model::<GetUser>()
    .one(&ctx.db_conn)
    .await
    .expect("could not find user");

  assert!(user.is_some());

  ctx.delete().await;

  Ok(())
}

fn create_user_active_model() -> schema::users::ActiveModel {
  let user = dto::user::CreateUser {
    username: "ahas".to_owned(),
    age: Nullable::Null,
    role: "admin".to_owned(),
    is_active: Optional(None),
  }
  .into_active_model();

  assert_eq!(
    user,
    schema::users::ActiveModel {
      username: Set("ahas".to_owned()),
      age: Set(None),
      role: Set("admin".to_owned()),
      is_active: NotSet,
      ..Default::default()
    }
  );

  user
}

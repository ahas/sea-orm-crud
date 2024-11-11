use crate::common::schema::users::ActiveModel;
use sea_orm::{DeriveIntoActiveModel, FromQueryResult};
use sea_orm_crud::nullable::Nullable;
use sea_orm_crud::optional::Optional;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
  pub username: String,
  pub age: Nullable<i32>,
  pub role: String,
  pub is_active: Optional<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
  pub username: Optional<String>,
  pub age: Nullable<i32>,
  pub role: Optional<String>,
  pub is_active: Optional<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct GetUser {
  pub username: String,
  pub age: Nullable<i32>,
  pub role: String,
  pub is_active: bool,
  pub created_at: String,
  pub updated_at: String,
}

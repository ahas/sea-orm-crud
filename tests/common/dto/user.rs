use crate::common::schema::users::ActiveModel;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::prelude::*;
use sea_orm::{DeriveIntoActiveModel, FromQueryResult};
use sea_orm_crud::nullable::Nullable;
use sea_orm_crud::optional::Optional;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
  pub username: String,
  #[serde(default)]
  pub age: Nullable<i32>,
  pub role: String,
  pub is_active: Optional<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
  pub username: Optional<String>,
  #[serde(default)]
  pub age: Nullable<i32>,
  pub role: Optional<String>,
  pub is_active: Optional<bool>,
}

#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct UserListItem {
  pub username: String,
  #[serde(default)]
  pub age: Nullable<i32>,
  pub role: String,
  pub is_active: bool,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct GetUser {
  pub username: String,
  #[serde(default)]
  pub age: Nullable<i32>,
  pub role: String,
  pub is_active: bool,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCreated {
  pub username: String,
}

impl IntoResponse for UserCreated {
  fn into_response(self) -> axum::response::Response {
    (StatusCode::CREATED, Json(self)).into_response()
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdated;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDeleted;

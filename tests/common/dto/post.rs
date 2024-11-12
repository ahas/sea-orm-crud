use crate::common::schema::posts::ActiveModel;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::DeriveIntoActiveModel;
use sea_orm::{prelude::*, FromQueryResult};
use sea_orm_crud::optional::Optional;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
  pub id: Optional<i32>,
  pub title: String,
  pub content: Optional<String>,
  pub status: Optional<String>,
  pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePost {
  pub id: Optional<i32>,
  pub title: Optional<String>,
  pub content: Optional<String>,
  pub status: Optional<String>,
  pub username: Optional<String>,
}

#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct PostListItem {
  pub id: i32,
  pub title: String,
  pub status: String,
  pub username: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct GetPost {
  pub id: i32,
  pub title: String,
  pub content: String,
  pub status: String,
  pub username: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCreated {
  pub id: i32,
}

impl IntoResponse for PostCreated {
  fn into_response(self) -> axum::response::Response {
    (StatusCode::CREATED, Json(self)).into_response()
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostUpdated;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDeleted;

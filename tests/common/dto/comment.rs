use crate::common::schema::comments::ActiveModel;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::{DeriveIntoActiveModel, FromQueryResult};
use sea_orm_crud::optional::Optional;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CreateComment {
  pub id: Optional<i32>,
  pub post: i32,
  pub content: Optional<String>,
  pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UpdateComment {
  pub id: Optional<i32>,
  pub post: Optional<i32>,
  pub content: Optional<String>,
  pub username: Optional<String>,
}

#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct CommentListItem {
  pub id: i32,
  pub post: i32,
  pub content: String,
  pub username: String,
}

#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct GetComment {
  pub id: i32,
  pub post: i32,
  pub content: String,
  pub username: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentCreated {
  pub id: i32,
}

impl IntoResponse for CommentCreated {
  fn into_response(self) -> axum::response::Response {
    (StatusCode::CREATED, Json(self)).into_response()
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentUpdated;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDeleted;

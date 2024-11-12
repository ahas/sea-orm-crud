use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use sea_orm::{DatabaseConnection, DbErr};

use super::TestContext;

pub mod comment;
pub mod post;
pub mod user;

#[derive(Clone)]
pub struct AppState {
  pub db_conn: DatabaseConnection,
}

pub enum AppError {
  DatabaseError,
  DuplicateItems,
  NotFound,
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
      Self::DuplicateItems => StatusCode::CONFLICT.into_response(),
      Self::NotFound => StatusCode::NOT_FOUND.into_response(),
    }
  }
}

impl From<DbErr> for AppError {
  fn from(err: DbErr) -> Self {
    match err {
      DbErr::RecordNotFound(_) => AppError::NotFound,
      DbErr::RecordNotInserted => AppError::DuplicateItems,
      _ => AppError::DatabaseError,
    }
  }
}

pub fn new(ctx: &TestContext) -> Router {
  Router::new()
    .nest("/users", user::new())
    .nest("/comments", comment::new())
    .nest("/posts", post::new())
    .with_state(AppState {
      db_conn: ctx.db_conn.clone(),
    })
}

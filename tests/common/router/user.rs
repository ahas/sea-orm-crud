use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{routing, Json, Router};
use sea_orm::{EntityTrait, IntoActiveModel};

use crate::common::dto::user::*;
use crate::common::schema::users;

use super::{AppError, AppState};

async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
  Ok(Json(users::Entity::find().into_model::<UserListItem>().all(&state.db_conn).await?))
}

async fn get(
  State(state): State<AppState>,
  Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {
  Ok(Json(
    users::Entity::find_by_id(username)
      .into_model::<GetUser>()
      .one(&state.db_conn)
      .await?
      .ok_or(AppError::NotFound)?,
  ))
}

async fn create(
  State(state): State<AppState>,
  Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
  let username = body.username.clone();
  users::Entity::insert(body.into_active_model()).exec(&state.db_conn).await?;

  Ok(UserCreated { username })
}

async fn update(
  State(state): State<AppState>,
  Json(body): Json<UpdateUser>,
) -> Result<impl IntoResponse, AppError> {
  users::Entity::update(body.into_active_model()).exec(&state.db_conn).await?;
  Ok(Json(UserUpdated))
}

async fn delete(
  State(state): State<AppState>,
  Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {
  users::Entity::delete_by_id(username).exec(&state.db_conn).await?;
  Ok(Json(UserDeleted))
}

pub fn new() -> Router<AppState> {
  Router::new()
    .route("/", routing::get(list))
    .route("/:username", routing::get(get))
    .route("/", routing::post(create))
    .route("/:username", routing::patch(update))
    .route("/:username", routing::delete(delete))
}

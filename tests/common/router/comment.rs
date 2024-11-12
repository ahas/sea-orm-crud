use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{routing, Json, Router};
use sea_orm::{EntityTrait, IntoActiveModel};

use crate::common::dto::comment::*;
use crate::common::schema::comments;

use super::{AppError, AppState};

async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
  Ok(Json(comments::Entity::find().into_model::<CommentListItem>().all(&state.db_conn).await?))
}

async fn get(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<GetComment>, AppError> {
  Ok(Json(
    comments::Entity::find_by_id(id)
      .into_model::<GetComment>()
      .one(&state.db_conn)
      .await?
      .ok_or(AppError::NotFound)?,
  ))
}

async fn create(
  State(state): State<AppState>,
  Json(body): Json<CreateComment>,
) -> Result<CommentCreated, AppError> {
  let result = comments::Entity::insert(body.into_active_model()).exec(&state.db_conn).await?;

  Ok(CommentCreated {
    id: result.last_insert_id,
  })
}

async fn update(
  State(state): State<AppState>,
  Json(body): Json<UpdateComment>,
) -> Result<Json<CommentUpdated>, AppError> {
  comments::Entity::update(body.into_active_model()).exec(&state.db_conn).await?;
  Ok(Json(CommentUpdated))
}

async fn delete(
  State(state): State<AppState>,
  Path(id): Path<i32>,
) -> Result<Json<CommentDeleted>, AppError> {
  comments::Entity::delete_by_id(id).exec(&state.db_conn).await?;
  Ok(Json(CommentDeleted))
}

pub fn new() -> Router<AppState> {
  Router::new()
    .route("/", routing::get(list))
    .route("/:id", routing::get(get))
    .route("/", routing::post(create))
    .route("/:id", routing::patch(update))
    .route("/:id", routing::delete(delete))
}

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{routing, Json, Router};
use sea_orm::{EntityTrait, IntoActiveModel};

use crate::common::dto::post::*;
use crate::common::schema::posts;

use super::{AppError, AppState};

async fn list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
  Ok(Json(posts::Entity::find().into_model::<PostListItem>().all(&state.db_conn).await?))
}

async fn get(State(state): State<AppState>, Path(id): Path<i32>) -> Result<impl IntoResponse, AppError> {
  Ok(Json(
    posts::Entity::find_by_id(id)
      .into_model::<GetPost>()
      .one(&state.db_conn)
      .await?
      .ok_or(AppError::NotFound)?,
  ))
}

async fn create(
  State(state): State<AppState>,
  Json(body): Json<CreatePost>,
) -> Result<impl IntoResponse, AppError> {
  let result = posts::Entity::insert(body.into_active_model()).exec(&state.db_conn).await?;

  Ok(PostCreated {
    id: result.last_insert_id,
  })
}

async fn update(
  State(state): State<AppState>,
  Json(body): Json<UpdatePost>,
) -> Result<Json<PostUpdated>, AppError> {
  posts::Entity::update(body.into_active_model()).exec(&state.db_conn).await?;
  Ok(Json(PostUpdated))
}

async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<PostDeleted>, AppError> {
  posts::Entity::delete_by_id(id).exec(&state.db_conn).await?;
  Ok(Json(PostDeleted))
}

pub fn new() -> Router<AppState> {
  Router::new()
    .route("/", routing::get(list))
    .route("/:id", routing::get(get))
    .route("/", routing::post(create))
    .route("/:id", routing::patch(update))
    .route("/:id", routing::delete(delete))
}

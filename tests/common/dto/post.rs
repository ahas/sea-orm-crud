use crate::common::schema::posts::ActiveModel;
use sea_orm::DeriveIntoActiveModel;
use sea_orm_crud::optional::Optional;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
  pub id: Optional<i32>,
  pub title: String,
  pub status: Optional<String>,
  pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePost {
  pub id: Optional<i32>,
  pub title: Optional<String>,
  pub status: Optional<String>,
  pub username: Optional<String>,
}

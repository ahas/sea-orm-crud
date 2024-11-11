use crate::common::schema::comments::ActiveModel;
use sea_orm::{ActiveModelTrait, DeriveIntoActiveModel};
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

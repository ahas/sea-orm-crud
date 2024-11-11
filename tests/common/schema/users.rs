use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub username: String,
  pub age: Option<i32>,
  pub role: String,
  pub is_active: bool,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::posts::Entity")]
  Posts,
  #[sea_orm(has_many = "super::comments::Entity")]
  Comments,
}

impl Related<super::posts::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Posts.def()
  }
}

impl Related<super::comments::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Comments.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

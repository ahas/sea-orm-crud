use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub title: String,
  pub content: String,
  pub status: String,
  pub username: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::comments::Entity")]
  Comments,
  #[sea_orm(
    belongs_to = "super::users::Entity",
    from = "Column::Username",
    to = "super::users::Column::Username"
  )]
  User,
}

impl Related<super::comments::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Comments.def()
  }
}

impl Related<super::users::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

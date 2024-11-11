use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub post: i32,
  pub content: String,
  pub username: String,
  pub created_at: DateTime,
  pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(belongs_to = "super::posts::Entity", from = "Column::Post", to = "super::posts::Column::Id")]
  Post,
  #[sea_orm(
    belongs_to = "super::users::Entity",
    from = "Column::Username",
    to = "super::users::Column::Username"
  )]
  User,
}

impl Related<super::posts::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Post.def()
  }
}

impl Related<super::users::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

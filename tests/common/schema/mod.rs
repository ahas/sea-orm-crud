pub mod comments;
pub mod posts;
pub mod users;

use sea_orm::prelude::Expr;
use sea_orm::sea_query::{ColumnDef, ForeignKey, Table};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, ForeignKeyAction};

pub async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
  let builder = db.get_database_backend();

  let stmt = Table::create()
    .table(users::Entity)
    .col(ColumnDef::new(users::Column::Username).string().not_null().primary_key())
    .col(ColumnDef::new(users::Column::Age).integer())
    .col(ColumnDef::new(users::Column::Role).string().not_null())
    .col(ColumnDef::new(users::Column::IsActive).boolean().default(false).not_null())
    .col(ColumnDef::new(users::Column::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
    .col(ColumnDef::new(users::Column::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
    .to_owned();

  db.execute(builder.build(&stmt)).await?;

  let stmt = Table::create()
    .table(posts::Entity)
    .col(ColumnDef::new(posts::Column::Id).integer().not_null().primary_key().auto_increment())
    .col(ColumnDef::new(posts::Column::Title).string().not_null().default(""))
    .col(ColumnDef::new(posts::Column::Content).text().not_null())
    .col(ColumnDef::new(posts::Column::Status).string().not_null().default("draft"))
    .col(ColumnDef::new(posts::Column::Username).string().not_null())
    .col(ColumnDef::new(users::Column::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
    .col(ColumnDef::new(users::Column::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
    .foreign_key(
      ForeignKey::create()
        .name("posts_username_fkey")
        .from(posts::Entity, posts::Column::Username)
        .to(users::Entity, users::Column::Username)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade),
    )
    .to_owned();

  db.execute(builder.build(&stmt)).await?;

  let stmt = Table::create()
    .table(comments::Entity)
    .col(ColumnDef::new(comments::Column::Id).integer().not_null().primary_key().auto_increment())
    .col(ColumnDef::new(comments::Column::Post).integer().not_null())
    .col(ColumnDef::new(comments::Column::Content).text().default(""))
    .col(ColumnDef::new(comments::Column::Username).string().not_null())
    .col(ColumnDef::new(users::Column::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null())
    .col(ColumnDef::new(users::Column::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
    .foreign_key(
      ForeignKey::create()
        .name("comments_post_fkey")
        .from(comments::Entity, comments::Column::Post)
        .to(posts::Entity, posts::Column::Id)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade),
    )
    .foreign_key(
      ForeignKey::create()
        .name("comments_username_fkey")
        .from(comments::Entity, comments::Column::Username)
        .to(users::Entity, users::Column::Username)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade),
    )
    .to_owned();

  db.execute(builder.build(&stmt)).await?;

  Ok(())
}

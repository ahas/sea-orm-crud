use sea_orm::sea_query::{Table, TableCreateStatement};
use sea_orm::{
  ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbConn, DbErr, EntityTrait,
  ExecResult, Schema,
};

pub async fn setup(base_url: &str, _db_name: &str) -> DatabaseConnection {
  let mut options: ConnectOptions = base_url.into();
  options.sqlx_logging(false);

  Database::connect(options).await.unwrap()
}

pub async fn tear_down(_base_url: &str, _db_name: &str) {}

pub async fn create_table<E>(
  db: &DbConn,
  create: &TableCreateStatement,
  entity: E,
) -> Result<ExecResult, DbErr>
where
  E: EntityTrait,
{
  let builder = db.get_database_backend();
  let schema = Schema::new(builder);
  assert_eq!(builder.build(&schema.create_table_from_entity(entity)), builder.build(create));

  create_table_without_asserts(db, create).await
}

pub async fn create_table_without_asserts(
  db: &DbConn,
  create: &TableCreateStatement,
) -> Result<ExecResult, DbErr> {
  let builder = db.get_database_backend();

  if builder != DbBackend::Sqlite {
    let stmt =
      builder.build(Table::drop().table(create.get_table_name().unwrap().clone()).if_exists().cascade());
    db.execute(stmt).await?;
  }

  db.execute(builder.build(create)).await
}

pub mod dto;
pub mod schema;
pub mod setup;

use sea_orm::DatabaseConnection;

pub struct TestContext {
  pub db_conn: DatabaseConnection,

  base_url: String,
  db_name: String,
}

impl TestContext {
  pub async fn new(test_name: &str) -> Self {
    dotenv::from_filename(".env.local").ok();
    dotenv::from_filename(".env").ok();

    let base_url = "sqlite::memory:".to_owned();
    let db_conn: DatabaseConnection = setup::setup(&base_url, test_name).await;

    Self {
      db_conn,
      base_url,
      db_name: test_name.to_string(),
    }
  }

  pub async fn delete(&self) {
    setup::tear_down(&self.base_url, &self.db_name).await;
  }
}

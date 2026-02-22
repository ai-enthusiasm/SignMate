use std::sync::Arc;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use tokio::sync::Mutex;

pub type DbConnection = Arc<Mutex<Surreal<Db>>>;

/// Initialize a local SurrealDB connection using RocksDb for persistent storage.
/// The database files will be stored in the app's data directory.
pub async fn init_db(app_data_dir: &str) -> Result<DbConnection, String> {
  let db_path = format!("{}/signmate.db", app_data_dir);

  let db = Surreal::new::<RocksDb>(&db_path)
    .await
    .map_err(|e| format!("Failed to connect to SurrealDB: {}", e))?;

  // Select namespace and database
  db.use_ns("signmate")
    .use_db("main")
    .await
    .map_err(|e| format!("Failed to select namespace/database: {}", e))?;

  // Create tables if they don't exist
  create_tables(&db).await?;

  Ok(Arc::new(Mutex::new(db)))
}

/// Define tables and schema on first run
async fn create_tables(db: &Surreal<Db>) -> Result<(), String> {
  db.query(
    "
    DEFINE TABLE IF NOT EXISTS vocabulary SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS word        ON vocabulary TYPE string;
    DEFINE FIELD IF NOT EXISTS topic       ON vocabulary TYPE string;
    DEFINE FIELD IF NOT EXISTS region      ON vocabulary TYPE string;
    DEFINE FIELD IF NOT EXISTS video_path  ON vocabulary TYPE string;
    DEFINE FIELD IF NOT EXISTS description ON vocabulary TYPE string;

    DEFINE TABLE IF NOT EXISTS user_progress SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS vocab_id       ON user_progress TYPE string;
    DEFINE FIELD IF NOT EXISTS status         ON user_progress TYPE string;
    DEFINE FIELD IF NOT EXISTS interval       ON user_progress TYPE int;
    DEFINE FIELD IF NOT EXISTS ease_factor    ON user_progress TYPE float;
    DEFINE FIELD IF NOT EXISTS next_review_at ON user_progress TYPE int;

    DEFINE TABLE IF NOT EXISTS user_settings SCHEMAFULL;
    DEFINE FIELD IF NOT EXISTS key       ON user_settings TYPE string;
    DEFINE FIELD IF NOT EXISTS value     ON user_settings TYPE string;
    DEFINE FIELD IF NOT EXISTS device_id ON user_settings TYPE string;
    ",
  )
  .await
  .map_err(|e| format!("Failed to create tables: {}", e))?;

  Ok(())
}

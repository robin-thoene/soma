use dirs::data_dir;
use sqlx::{SqlitePool, migrate::MigrateDatabase};
use std::fs;

pub async fn init_db_connection() -> Result<SqlitePool, sqlx::Error> {
    let app_data_dir = data_dir()
        .expect("expect the data dir to be resolvable")
        .join("soma");
    fs::create_dir_all(&app_data_dir).expect("app data dir must be able to be created");
    let db_path = &format!("{}/soma_db.sqlite", app_data_dir.display());
    if !sqlx::Sqlite::database_exists(db_path).await? {
        sqlx::Sqlite::create_database(db_path).await?;
    }
    let pool = SqlitePool::connect(db_path).await?;
    sqlx::migrate!("src/outbound/database/migrations")
        .run(&pool)
        .await?;
    Ok(pool)
}

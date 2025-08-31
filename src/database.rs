use anyhow::{Result, anyhow};
use sea_orm::{Database, DbConn};

pub async fn connect(database_url: &str, _max_connections: u32) -> Result<DbConn> {
    // sqlite://path?mode=rwc
    let url = format!("sqlite://{}?mode=rwc", database_url);

    // Tạo connection pool
    let db = Database::connect(&url)
        .await
        .map_err(|e| anyhow!("Connection error: {e}"))?;

    println!("Kết nối SQLite thành công: {}", database_url);

    Ok(db)
}

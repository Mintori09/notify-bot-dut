use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::{env, fs};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub teloxide_token: String,
    pub chat_id: i64,
    pub filter: Option<Vec<String>>,
}

impl Config {
    pub fn init() -> Self {
        dotenv().ok();

        let db_url = ProjectDirs::from("com", "", "notify-bot-dut")
            .map(|dirs| {
                let config_dir = dirs.config_dir().to_path_buf();
                fs::create_dir_all(&config_dir)
                    .expect("Failed to create config directory");
                format!(
                    "sqlite://{}?mode=rwc",
                    config_dir.join("college.db").display()
                )
            })
            .unwrap_or_else(|| "sqlite://college.db?mode=rwc".to_string());

        Self {
            teloxide_token: env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN must be set"),
            database_url: db_url,
            chat_id: env::var("CHAT_ID")
                .expect("CHAT_ID must be set")
                .parse::<i64>()
                .unwrap(),
            filter: env::var("FILTER_NOTICE")
                .ok()
                .map(|s| s.split(',').map(|item| item.trim().to_string()).collect()),
        }
    }
}

pub async fn connect(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url)
        .await
        .map_err(|e| anyhow!("Connection error: {e}"))?;

    println!("Connected to SQLite: {}", database_url);
    Ok(pool)
}

pub async fn ensure_schema(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS notice_sent (
            id             INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
            main_category  TEXT     NOT NULL,
            external_id    TEXT     NOT NULL,
            published_date DATE     NULL,
            body           TEXT     NULL,
            title          TEXT     NOT NULL,
            sent_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            sent_ok        INTEGER  NOT NULL DEFAULT 0,
            CHECK (main_category IN ('Training','ClassNotice','StudentAffairs','Tuition')),
            CHECK (length(external_id) > 0),
            CHECK (body IS NULL OR length(trim(body)) > 0),
            CHECK (length(trim(title)) >= 3),
            UNIQUE (main_category, external_id)
        )"#,
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow!("Schema error: {e}"))?;
    Ok(())
}

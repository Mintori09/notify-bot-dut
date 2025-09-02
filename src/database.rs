use anyhow::{Result, anyhow};
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DbConn};
use std::env;

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

        Self {
            teloxide_token: env::var("TELOXIDE_TOKEN").expect("TELEXIDE_TOKEN must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
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

pub async fn connect(database_url: &str, max_connections: u32) -> Result<DbConn> {
    let mut connect_options = ConnectOptions::new(database_url.to_owned());

    connect_options
        .max_connections(max_connections)
        .sqlx_logging(true);

    let db = Database::connect(connect_options)
        .await
        .map_err(|e| anyhow!("Connection error: {e}"))?;

    println!("Connected to Postgres: {}", database_url);
    Ok(db)
}

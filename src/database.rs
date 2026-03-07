use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::{fs, path::PathBuf};

/// Raw JSON structure — mirrors config.json on disk
#[derive(Deserialize, Debug)]
struct ConfigFile {
    teloxide_token: Option<String>,
    chat_id: Option<i64>,
    filter_notice: Option<Vec<String>>,
}

/// Runtime config (merged from file + CLI overrides)
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub teloxide_token: String,
    pub chat_id: i64,
    pub filter: Option<Vec<String>>,
}

/// Path to ~/.config/notify-bot-dut/
pub fn config_dir() -> PathBuf {
    ProjectDirs::from("com", "", "notify-bot-dut")
        .map(|d| d.config_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".config/notify-bot-dut"))
}

/// Path to the config.json file
pub fn config_file_path() -> PathBuf {
    config_dir().join("config.json")
}

impl Config {
    /// Load config from ~/.config/notify-bot-dut/config.json.
    /// If the file does not exist, a template is created and the process exits.
    pub fn load() -> Self {
        let config_dir = config_dir();
        let db_url = format!(
            "sqlite://{}?mode=rwc",
            config_dir.join("college.db").display()
        );

        let path = config_file_path();

        if !path.exists() {
            fs::create_dir_all(&config_dir).expect("Failed to create config directory");
            let template = r#"{
  "teloxide_token": "",
  "chat_id": 0,
  "filter_notice": ["23.Nh16", "23.Nh44"]
}
"#;
            fs::write(&path, template).expect("Failed to write config template");
            eprintln!(
                "✗ Config file not found. A template has been created at:\n  {}\n\nPlease fill in your bot token and chat ID, then run again.",
                path.display()
            );
            std::process::exit(1);
        }

        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

        let file: ConfigFile = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("Invalid JSON in {}: {}", path.display(), e));

        let teloxide_token = file
            .teloxide_token
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| {
                eprintln!(
                    "✗ `teloxide_token` is empty in {}.\nPlease add your bot token.",
                    path.display()
                );
                std::process::exit(1);
            });

        let chat_id = file.chat_id.filter(|&n| n != 0).unwrap_or_else(|| {
            eprintln!(
                "✗ `chat_id` is 0 or missing in {}.\nPlease set your Telegram group/channel ID.",
                path.display()
            );
            std::process::exit(1);
        });

        Self {
            database_url: db_url,
            teloxide_token,
            chat_id,
            filter: file.filter_notice,
        }
    }

    /// Apply CLI flag overrides (only non-None values overwrite the loaded config).
    pub fn apply_overrides(
        &mut self,
        token: Option<String>,
        chat_id: Option<i64>,
        filter: Option<Vec<String>>,
    ) {
        if let Some(t) = token {
            self.teloxide_token = t;
        }
        if let Some(c) = chat_id {
            self.chat_id = c;
        }
        if let Some(f) = filter {
            if !f.is_empty() {
                self.filter = Some(f);
            }
        }
    }
}

pub async fn connect(database_url: &str) -> Result<SqlitePool> {
    // Ensure the directory for the DB file exists
    if let Some(path) = database_url
        .trim_start_matches("sqlite://")
        .split('?')
        .next()
    {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent).ok();
        }
    }

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

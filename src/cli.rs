// src/cli.rs
use std::path::PathBuf;

use clap::Parser;
use directories::BaseDirs;
use once_cell::sync::Lazy;

/// Parse once and expose globally.
pub static CLI: Lazy<Cli> = Lazy::new(parse_args);

/// Remindee CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the SQLite database file (tries to create if not exists)
    ///
    /// - Flag: --database / -d
    /// - Env:  REMINDEE_DB
    #[arg(
        short,
        long,
        env = "REMINDEE_DB",
        value_name = "FILE",
        default_value_os_t = default_database_file()
    )]
    pub database: PathBuf,

    /// Bot token (or set BOT_TOKEN env var)
    ///
    /// - Flag: --token / -t
    /// - Env:  BOT_TOKEN
    #[arg(short, long, value_name = "BOT TOKEN", env = "BOT_TOKEN")]
    pub token: String,

    /// Maximum number of connections to the SQLite database
    ///
    /// - Flag: --sqlite-max-connections / -s
    /// - Env:  SQLITE_MAX_CONNECTIONS
    #[arg(
        short,
        long,
        env = "SQLITE_MAX_CONNECTIONS",
        value_name = "NUMBER",
        default_value_t = 1u32
    )]
    pub sqlite_max_connections: u32,
}

/// Parse command-line arguments.
pub fn parse_args() -> Cli {
    Cli::parse()
}

/// Build a sensible cross-platform default database path.
///
/// * Android:  ./remindee_db.sqlite (current working directory)
/// * Others:   <platform data dir>/remindee_db.sqlite
fn default_database_file() -> PathBuf {
    let db_name = "remindee_db.sqlite";

    if cfg!(target_os = "android") {
        PathBuf::from(db_name)
    } else if cfg!(target_os = "linux") {
        // Follow XDG spec manually
        if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
            PathBuf::from(xdg_data_home).join(db_name)
        } else {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join(".local/share")
                .join(db_name)
        }
    } else {
        match BaseDirs::new() {
            Some(base) => base.data_dir().join(db_name),
            None => PathBuf::from(db_name),
        }
    }
}

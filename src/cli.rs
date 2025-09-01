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
    #[arg(short, long, env = "REMINDEE_DB", value_name = "FILE")]
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

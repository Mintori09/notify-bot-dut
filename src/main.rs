use anyhow::{Result, anyhow};

mod bot;
mod cli;
mod database;
mod fetch;
mod scheduler;

#[tokio::main]
async fn main() -> Result<()> {
    bot::run().await.map_err(|e| anyhow!("Error at main: {e}"))
}

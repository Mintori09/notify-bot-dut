use anyhow::{Result, anyhow};

use notify_bot_dut::bot;

#[tokio::main]
async fn main() -> Result<()> {
    bot::run().await.map_err(|e| anyhow!("Error at main: {e}"))
}

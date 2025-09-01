use notify_bot_dut::{bot, scheduler::run_scheduler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_scheduler(|| async { bot::run().await }).await;

    Ok(())
}

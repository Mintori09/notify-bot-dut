use notify_bot_dut::{
    bot,
    database::{self, Config},
    scheduler::run_scheduler,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init();
    let db = database::connect(&config.database_url, 5).await?;
    run_scheduler(|| async { bot::run(&db).await }).await;

    Ok(())
}

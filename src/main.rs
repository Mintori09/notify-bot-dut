use notify_bot_dut::{
    bot,
    database::{self, Config},
    scheduler::run_scheduler,
    utils::wait_for_internet,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init();

    let db = database::connect(&config.database_url, 5).await?;

    run_scheduler(|| async {
        wait_for_internet(300).await;
        bot::run(&db).await
    })
    .await;

    Ok(())
}

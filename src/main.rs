use notify_bot_dut::{
    bot,
    cli::{self, Cli, SubCommand},
    database::{self, Config},
    scheduler::run_scheduler,
    utils::wait_for_internet,
};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(SubCommand::Config) => {
            cli::open_config();
        }
        Some(SubCommand::InstallService) => {
            cli::install_service();
        }
        None => {
            // Load config from file, then apply CLI flag overrides
            let mut config = Config::load();
            config.apply_overrides(
                cli.token,
                cli.chat_id,
                if cli.filter.is_empty() {
                    None
                } else {
                    Some(cli.filter)
                },
            );

            println!("Config: {:?}", config);

            let db = database::connect(&config.database_url).await?;
            database::ensure_schema(&db).await?;

            run_scheduler(|| async {
                wait_for_internet(300).await;
                bot::run(&db, &config).await
            })
            .await;
        }
    }

    Ok(())
}

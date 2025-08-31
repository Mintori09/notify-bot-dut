use crate::fetch::http_client;
use crate::{cli::CLI, database, fetch, scheduler};
use anyhow::Result;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::{Bot, types::ChatId};

pub async fn run() -> Result<()> {
    let db_file = CLI.database.to_string_lossy().to_string();
    let db = database::connect(&db_file, CLI.sqlite_max_connections).await?;

    let bot = Bot::new(CLI.token.clone());

    let client = Arc::new(http_client());

    let chat_id = ChatId(123456789);

    scheduler::run_scheduler(|| {
        let bot = bot.clone();
        let db = db.clone();
        let client = Arc::clone(&client);
        async move {
            match fetch::fetch_notices(&client).await {
                Ok(notices) => {
                    for n in notices {
                        // TODO: check DB xem đã gửi chưa
                        // database::check_and_insert(&db, &n).await?;

                        if let Err(e) = bot.send_message(chat_id, &n).await {
                            eprintln!("❌ Lỗi gửi Telegram: {e}");
                        }
                    }
                }
                Err(e) => eprintln!("❌ Lỗi fetch: {e}"),
            }
        }
    })
    .await;

    Ok(())
}

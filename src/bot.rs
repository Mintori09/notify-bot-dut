use std::sync::Arc;
use std::time::Duration;

use crate::controller::check_and_insert;
use crate::database::{self, Config};
use crate::entity::NoticeSent;
use crate::fetch::{fetch_all_notices, http_client};
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::{Bot, types::ChatId};
use tokio::time::sleep;

pub async fn run() -> Result<()> {
    // 1. Load config
    let config = Config::init();

    // 2. Connect to DB
    let db = database::connect(&config.database_url, 5).await?;

    // 3. Init HTTP client + Telegram bot
    let client = Arc::new(http_client());
    let bot = Bot::new(&config.teloxide_token);
    let chat_id = ChatId(config.chat_id);

    // 4. Fetch notices from source
    let notices: Vec<NoticeSent> = fetch_all_notices(&client).await?;

    // 5. Insert + send
    for notice in notices {
        match check_and_insert(&db, &notice).await {
            Ok(true) => {
                sleep(Duration::from_secs(1)).await;
                if let Err(err) = send_notice(&bot, chat_id, &notice).await {
                    eprintln!("Failed to send notice {}: {}", notice.title, err);
                }
            }
            Ok(false) => {}
            Err(err) => {
                eprintln!("Failed to check/insert {}: {}", notice.title, err);
            }
        }
    }

    Ok(())
}

pub async fn send_notice(bot: &Bot, chat_id: ChatId, notice: &NoticeSent) -> Result<()> {
    let date_str = notice
        .published_date
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "N/A".to_string());

    let body_str = notice
        .body
        .as_ref()
        .map(|b| html_escape::encode_text(b).to_string())
        .unwrap_or_else(|| "No details".to_string());

    let sent_at_str = notice.sent_at.format("%Y-%m-%d %H:%M:%S").to_string();

    let text = format!(
        "#{category}\n\
         <b>{title}</b>\n\
         <b>Date:</b> {date}\n\
         <b>Details:</b>\n{body}\n\
         <i>Sent at: {sent_at}</i>",
        category = notice.main_category,
        title = html_escape::encode_text(&notice.title),
        date = date_str,
        body = body_str,
        sent_at = sent_at_str,
    );

    bot.send_message(chat_id, text)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await?;

    Ok(())
}

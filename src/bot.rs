use std::sync::Arc;

use crate::controller::{check_and_insert, get_unsent, mark_as_sent};
use crate::database::Config;
use crate::entity::NoticeSent;
use crate::fetch::{fetch_all_notices, http_client};
use anyhow::Result;
use sqlx::SqlitePool;
use teloxide::prelude::*;
use teloxide::{Bot, types::ChatId};

pub async fn run(db: &SqlitePool, config: &Config) -> Result<()> {
    use tokio::time::{Duration, sleep};
    let client = Arc::new(http_client());
    let (bot, chat_id) = build_bot(&config)?;

    let notices: Vec<NoticeSent> = fetch_all_notices(&client).await?;
    for notice in notices {
        if let Err(err) = check_and_insert(db, &notice, &config).await {
            eprintln!("Failed to insert {}: {}", notice.title, err);
        }
    }
    let unsent = get_unsent(db).await?;

    for notice in unsent {
        let mut retries = 0;
        loop {
            match send_notice(&bot, chat_id, &notice).await {
                Ok(_) => {
                    println!("Sent notice: {}", notice.title);
                    mark_as_sent(db, &notice).await?;
                    break;
                }
                Err(err) => {
                    retries += 1;
                    let err_str = err.to_string();
                    // Parse "Retry after N" from Telegram flood-control errors
                    let wait_secs = err_str
                        .to_lowercase()
                        .split("retry after")
                        .nth(1)
                        .and_then(|s| s.split_whitespace().next())
                        .and_then(|s| s.trim_end_matches('s').parse::<u64>().ok())
                        .map(|n| n + 1) // +1s buffer
                        .unwrap_or(5);
                    eprintln!(
                        "Failed to send '{}': {} (retry {}/5, waiting {}s)",
                        notice.title, err_str, retries, wait_secs
                    );
                    if retries >= 5 {
                        eprintln!("Giving up on '{}'", notice.title);
                        break;
                    }
                    sleep(Duration::from_secs(wait_secs)).await;
                    continue;
                }
            }
        }
    }
    Ok(())
}

pub fn build_bot(config: &Config) -> Result<(Bot, ChatId)> {
    let bot = Bot::new(&config.teloxide_token);
    let chat_id = ChatId(config.chat_id);
    Ok((bot, chat_id))
}

pub async fn send_notice(bot: &Bot, chat_id: ChatId, notice: &NoticeSent) -> Result<bool> {
    let date_str = notice
        .published_date
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "N/A".to_string());

    let body_str = notice
        .body
        .as_ref()
        .cloned()
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

    Ok(true)
}

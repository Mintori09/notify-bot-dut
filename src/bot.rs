use std::sync::Arc;
use std::time::Duration;

use crate::controller::check_and_insert;
use crate::database::{self, Config};
use crate::entity::NoticeSent;
use crate::fetch::{fetch_all_notices, http_client};
use anyhow::Result;
use sea_orm::DbConn;
use teloxide::{ApiError, RequestError, prelude::*};
use teloxide::{Bot, types::ChatId};
use tokio::time::sleep;

pub async fn run(db: &DbConn) -> Result<()> {
    // 1. Load config
    let config = Config::init();

    // 2. Init HTTP client + Telegram bot
    let client = Arc::new(http_client());
    let bot = Bot::new(&config.teloxide_token);
    let chat_id = ChatId(config.chat_id);

    // 3. Fetch notices from source
    let notices: Vec<NoticeSent> = fetch_all_notices(&client).await?;

    // 4. Insert + send
    for notice in notices {
        match check_and_insert(db, &notice).await {
            Ok(true) => {
                // new notice → try sending with retry
                loop {
                    match send_notice(&bot, chat_id, &notice).await {
                        Ok(_) => {
                            println!("✅ Sent notice: {}", notice.title);
                            break;
                        }
                        Err(err) => {
                            // Is this a teloxide RequestError?
                            if let Some(req_err) = err.downcast_ref::<RequestError>() {
                                match req_err {
                                    // Telegram said: wait N seconds (flood control)
                                    RequestError::RetryAfter(_secs) => {
                                        // `Seconds` is a newtype over u32: sleep for that many seconds
                                        // convert Seconds → u64
                                        sleep(Duration::from_secs(5)).await;
                                        continue;
                                    }
                                    // Other Telegram API errors: brief backoff and retry
                                    RequestError::Api(api_err) => {
                                        eprintln!(
                                            "⚠️ Telegram API error for '{}': {}",
                                            notice.title, api_err
                                        );
                                        sleep(Duration::from_secs(5)).await;
                                        continue;
                                    }
                                    // Network, IO, etc.: brief backoff and retry
                                    _ => {
                                        eprintln!(
                                            "⚠️ Request error for '{}': {}",
                                            notice.title, req_err
                                        );
                                        sleep(Duration::from_secs(5)).await;
                                        continue;
                                    }
                                }
                            }

                            // Not a teloxide RequestError (some other anyhow/source error): brief backoff
                            eprintln!(
                                "⚠️ Failed to send '{}': {} → retrying in 5s",
                                notice.title, err
                            );
                            sleep(Duration::from_secs(5)).await;
                            continue;
                        }
                    }
                }
            }
            Ok(false) => {
                // already exists → skip
            }
            Err(err) => {
                eprintln!("❌ Failed to check/insert {}: {}", notice.title, err);
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

    Ok(())
}

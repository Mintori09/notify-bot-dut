use crate::entity::NoticeSent;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::{Bot, types::ChatId};

pub async fn run() -> Result<()> {
    // let db_file = CLI.database.to_string_lossy().to_string();
    // let db = database::connect(&db_file, CLI.sqlite_max_connections).await?;
    //
    // let bot = Bot::new(CLI.token.clone());
    //
    // let client = Arc::new(http_client());
    //
    // let chat_id = ChatId(123456789);
    //
    // scheduler::run_scheduler(|| {
    //     let bot = bot.clone();
    //     let db = db.clone();
    //     let client = Arc::clone(&client);
    //     async move {
    //         match fetch::fetch_class_notice(&client).await {
    //             Ok(notices) => {
    //                 for n in notices {
    //                     // TODO: check DB xem đã gửi chưa
    //                     // database::check_and_insert(&db, &n).await?;
    //
    //                     if let Err(e) = bot.send_message(chat_id, &n).await {
    //                         eprintln!("❌ Lỗi gửi Telegram: {e}");
    //                     }
    //                 }
    //             }
    //             Err(e) => eprintln!("❌ Lỗi fetch: {e}"),
    //         }
    //     }
    // })
    // .await;

    Ok(())
}

pub async fn send_notice(bot: &Bot, chat_id: ChatId, notice: &NoticeSent) -> Result<()> {
    let text = format!(
        " <b>{title}</b>\n\
      <b>Date:</b> {date}\n\
      <b>Details:</b>\n{body}\n\
     <i>Sent at: {sent_at}</i>",
        title = html_escape::encode_text(&notice.title),
        date = notice.published_date,
        body = html_escape::encode_text(&notice.body),
        sent_at = notice.sent_at,
    );

    if let Err(err) = bot
        .send_message(chat_id, text)
        .parse_mode(ParseMode::Html)
        .await
    {
        eprintln!("❌ Failed to send Telegram notice: {err}");
        return Err(anyhow::anyhow!(err));
    }

    println!("✅ Sent notice: {}", notice.title);
    Ok(())
}

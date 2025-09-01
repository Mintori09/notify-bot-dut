use chrono::{NaiveDate, NaiveDateTime};
use notify_bot_dut::{
    bot::send_notice,
    entity::{Category, NoticeSent},
};
use std::env;
use teloxide::{prelude::*, types::ChatId};

#[tokio::test]
async fn test_send_notice() {
    let token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN not set");
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("CHAT_ID not set")
        .parse()
        .expect("CHAT_ID must be number");

    let bot = Bot::new(token);
    let chat_id = ChatId(chat_id);

    let notice = NoticeSent {
        id: 0,
        main_category: Category::ClassNotice,
        external_id: "test123".to_string(),
        published_date: Some(NaiveDate::parse_from_str("2025-08-31", "%Y-%m-%d").unwrap()),
        title: "Test Message from notify-bot-dut".to_string(),
        body: Some("This is a test notice sent via Telegram bot.".to_string()),
        sent_at: NaiveDateTime::parse_from_str("2025-08-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap(),
    };

    let result = send_notice(&bot, chat_id, &notice).await;
    assert!(result.is_ok(), "Message sending failed: {:?}", result.err());
}

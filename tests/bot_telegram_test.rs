use notify_bot_dut::{
    bot::send_notice,
    entity::{Category, NoticeSent},
};

#[tokio::test]
#[ignore]
async fn test_send_notice() {
    let token = match std::env::var("TELOXIDE_TOKEN") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("⚠️  Skipping test_send_notice: TELOXIDE_TOKEN not set");
            return;
        }
    };

    let chat_id: i64 = match std::env::var("CHAT_ID") {
        Ok(v) => match v.parse() {
            Ok(id) => id,
            Err(_) => {
                eprintln!("⚠️  Skipping test_send_notice: CHAT_ID must be number");
                return;
            }
        },
        Err(_) => {
            eprintln!("⚠️  Skipping test_send_notice: CHAT_ID not set");
            return;
        }
    };

    let bot = teloxide::Bot::new(token);
    let chat_id = teloxide::types::ChatId(chat_id);

    let notice = NoticeSent {
        id: 0,
        main_category: Category::Tuition,
        external_id: "test123".to_string(),
        published_date: Some(chrono::NaiveDate::parse_from_str("2025-08-31", "%Y-%m-%d").unwrap()),
        title: "Test Message from notify-bot-dut".to_string(),
        body: Some("This is a test notice sent via Telegram bot.".to_string()),
        sent_at: chrono::NaiveDateTime::parse_from_str("2025-08-31 23:59:59", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        sent_ok: 0,
    };

    let result = send_notice(&bot, chat_id, &notice).await;
    assert!(result.is_ok(), "Message sending failed: {:?}", result.err());
}

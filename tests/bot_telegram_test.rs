use notify_bot_dut::bot::send_notice;
use notify_bot_dut::entity::NoticeSent;
use std::env;
use teloxide::Bot;
use teloxide::types::ChatId;

#[tokio::test]
async fn test_send_notice() {
    // Đọc từ biến môi trường
    let token = env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN not set");
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("CHAT_ID not set")
        .parse()
        .expect("CHAT_ID must be number");

    let bot = Bot::new(token);
    let chat_id = ChatId(chat_id);

    // NoticeSent mẫu
    let notice = NoticeSent {
        id: 0,
        main_category: "ClassNotice".to_string(),
        external_id: "test123".to_string(),
        published_date: "2025-08-31".to_string(),
        title: "Test Message from notify-bot-dut".to_string(),
        body: "This is a test notice sent via Telegram bot.".to_string(),
        sent_at: "2025-08-31 23:59:59".to_string(),
    };

    // Gửi thử
    let result = send_notice(&bot, chat_id, &notice).await;
    assert!(result.is_ok(), "Message sending failed: {:?}", result.err());
}

use notify_bot_dut::{
    bot::send_notice,
    controller::check_and_insert,
    database::{self, Config},
    entity::{Category, NoticeSent},
    fetch::{fetch_all_notices, http_client},
};
use sea_orm::Database;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use teloxide::{Bot, types::ChatId};

#[tokio::test]
async fn test_send_notice() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // 1. Connect to DB
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let db = database::connect(&url, 1).await?;

    // 2. Build a unique title using a hash
    let mut hasher = Sha256::new();
    hasher.update(b"sdfjklsdfjli");
    let hash = format!("{:x}", hasher.finalize());
    let unique_title = format!("Hello PG {}", &hash[..12]);

    // 3. Create test notice
    let notice = NoticeSent::new(
        Some("2025-09-01".to_string()),
        unique_title,
        Some("Body PG".to_string()),
        Category::Training,
    );

    // 4. Load config
    let config = Config::init();

    // 5. Init HTTP + Telegram bot
    let _client = Arc::new(http_client()); // not used here
    let bot = Bot::new(&config.teloxide_token);
    let chat_id = ChatId(config.chat_id);

    // 6. Insert + send

    match check_and_insert(&db, &notice).await {
        Ok(true) => {
            let result = send_notice(&bot, chat_id, &notice).await;
            assert!(
                result.is_ok(),
                "❌ Failed to send notice: {:?}",
                result.err()
            );
        }
        Ok(false) => {
            panic!("Notice already existed, test needs unique external_id");
        }
        Err(err) => {
            panic!("❌ Failed to check/insert {}: {}", notice.title, err);
        }
    }

    Ok(())
}

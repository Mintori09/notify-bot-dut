use notify_bot_dut::{
    bot::send_notice,
    controller::check_and_insert,
    database::{self, Config},
    entity::{Category, NoticeSent},
    fetch::http_client,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use teloxide::{Bot, types::ChatId};

#[tokio::test]
#[ignore]
async fn test_send_notice() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let db = database::connect(&url, 1).await?;

    let mut hasher = Sha256::new();
    hasher.update(b"sdfjklsdfjli");
    let hash = format!("{:x}", hasher.finalize());
    let unique_title = format!("Hello PG {}", &hash[..12]);
    let notice = NoticeSent::new(
        Some("2025-09-01".to_string()),
        unique_title,
        Some("Body PG".to_string()),
        Category::Training,
    );

    let config = Config::init();
    let _client = Arc::new(http_client());
    let bot = Bot::new(&config.teloxide_token);
    let chat_id = ChatId(config.chat_id);

    match check_and_insert(&db, &notice, &config).await {
        Ok(true) => {
            let result = send_notice(&bot, chat_id, &notice).await;
            assert!(result.is_ok(), "Failed to send notice: {:?}", result.err());
        }
        Ok(false) => {
            panic!("Notice already existed, test needs unique external_id");
        }
        Err(err) => {
            panic!("Failed to check/insert {}: {}", notice.title, err);
        }
    }

    Ok(())
}

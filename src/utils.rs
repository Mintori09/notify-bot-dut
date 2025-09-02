use crate::database::Config;
use crate::entity::{Category, NoticeSent};
use anyhow::{Result, anyhow};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

pub async fn has_internet() -> bool {
    TcpStream::connect(("8.8.8.8", 53)).await.is_ok()
}

pub async fn wait_for_internet(interval: u64) {
    loop {
        if has_internet().await {
            println!("Internet connection detected");
            break;
        } else {
            println!("No internet, retrying in {interval} seconds...");
            sleep(Duration::from_secs(interval)).await;
        }
    }
}

pub async fn filter_notice(notice: &NoticeSent, config: &Config) -> Result<bool> {
    if notice.main_category != Category::ClassNotice {
        Ok(true)
    } else {
        let content = notice.title.clone();
        if let Some(filters) = &config.filter {
            for filter in filters {
                if content.contains(filter) {
                    return Ok(true);
                }
            }
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

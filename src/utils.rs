use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

/// Simple internet check: tries to connect to Google DNS
pub async fn has_internet() -> bool {
    TcpStream::connect(("8.8.8.8", 53)).await.is_ok()
}

/// Wait until internet is available, retrying every `interval` seconds
pub async fn wait_for_internet(interval: u64) {
    loop {
        if has_internet().await {
            println!("✅ Internet connection detected");
            break;
        } else {
            println!("🌐 No internet, retrying in {interval} seconds...");
            sleep(Duration::from_secs(interval)).await;
        }
    }
}

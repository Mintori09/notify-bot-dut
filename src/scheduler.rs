use tokio::time::{self, Duration};

pub async fn run_scheduler<F, Fut>(mut task: F)
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<()>>,
{
    let mut interval = time::interval(Duration::from_secs(3600));

    if let Err(err) = task().await {
        eprintln!("Scheduled task failed on startup: {}", err);
    }

    loop {
        interval.tick().await;
        if let Err(err) = task().await {
            eprintln!("Scheduled task failed: {}", err);
        }
    }
}

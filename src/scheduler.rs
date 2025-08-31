use tokio::time::{self, Duration};

pub async fn run_scheduler<F, Fut>(mut task: F)
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let mut interval = time::interval(Duration::from_secs(3600)); // 1 giờ
    loop {
        interval.tick().await;
        task().await;
    }
}

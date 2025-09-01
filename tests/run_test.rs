use anyhow::Result;
use chrono::NaiveDate;
use notify_bot_dut::{
    controller::check_and_insert,
    entity::{Category, NoticeSent},
    fetch::analysis_notice,
    scheduler::run_scheduler,
};
use sea_orm::Database;
use std::sync::{Arc, Mutex};
use tokio::time::Duration;

#[tokio::test]
async fn test_analysis_notice() {
    let html = r#"
        <div class="tbBox">
            <div class="tbBoxCaption">2025-09-01: Test Notice</div>
            <div class="tbBoxContent">Hello from test</div>
        </div>
    "#;

    let notices = analysis_notice(html, Category::Training).unwrap();
    assert_eq!(notices.len(), 1);
    assert_eq!(notices[0].title, "Test Notice");
    assert_eq!(
        notices[0].published_date,
        Some(NaiveDate::from_ymd_opt(2025, 9, 1).unwrap())
    );
    assert_eq!(notices[0].body.as_deref(), Some("Hello from test"));
}

#[tokio::test]
async fn test_check_and_insert_postgres() -> Result<()> {
    dotenv::dotenv().ok();

    // 1. Connect to Postgres (make sure migrations are applied!)
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&url).await?;

    // 2. Create a test notice
    let notice = NoticeSent {
        id: 0,
        main_category: Category::Training,
        external_id: "unique_test_id_123".to_string(),
        published_date: Some(NaiveDate::from_ymd_opt(2025, 9, 1).unwrap()),
        title: "Test Insert".to_string(),
        body: Some("This is a test".to_string()),
        sent_at: chrono::Utc::now().naive_utc(),
    };

    // 3. First insert should succeed
    let first = check_and_insert(&db, &notice).await?;
    assert!(first, "First insert should return true");

    // 4. Second insert with same external_id should be skipped
    let second = check_and_insert(&db, &notice).await?;
    assert!(!second, "Second insert should return false");

    Ok(())
}

#[tokio::test]
async fn test_scheduler_runs_task() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    // Run scheduler with short interval
    let handle = tokio::spawn(async move {
        run_scheduler(|| {
            let counter = counter_clone.clone();
            async move {
                *counter.lock().unwrap() += 1;
                Ok(())
            }
        })
        .await;
    });

    tokio::time::sleep(Duration::from_millis(2200)).await;
    handle.abort();

    let runs = *counter.lock().unwrap();
    assert!(runs >= 1, "Expected at least 1 run, got {}", runs);
}

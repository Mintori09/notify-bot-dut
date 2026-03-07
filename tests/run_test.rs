use anyhow::Result;
use chrono::NaiveDate;
use notify_bot_dut::{
    controller::check_and_insert,
    database::Config,
    entity::{Category, NoticeSent},
    fetch::analysis_notice,
};

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
#[ignore]
async fn test_check_and_insert_sqlite() -> Result<()> {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    notify_bot_dut::database::ensure_schema(&pool).await?;

    let notice = NoticeSent {
        id: 0,
        main_category: Category::Training,
        external_id: "unique_test_id_123".to_string(),
        published_date: Some(NaiveDate::from_ymd_opt(2025, 9, 1).unwrap()),
        title: "Test Insert".to_string(),
        body: Some("This is a test".to_string()),
        sent_at: chrono::Utc::now().naive_utc(),
        sent_ok: 0,
    };

    let config = Config::load();
    let first = check_and_insert(&pool, &notice, &config).await?;
    assert!(first, "First insert should return true");

    let second = check_and_insert(&pool, &notice, &config).await?;
    assert!(!second, "Second insert should return false");

    Ok(())
}

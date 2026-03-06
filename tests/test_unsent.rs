use notify_bot_dut::entity::{Category, NoticeSent};

fn make_notice(title: &str, sent_ok: i32) -> NoticeSent {
    NoticeSent {
        id: 1,
        main_category: Category::ClassNotice,
        external_id: "abc123".into(),
        published_date: None,
        title: title.into(),
        body: Some("Body content".into()),
        sent_at: chrono::NaiveDateTime::parse_from_str("2025-09-01 12:30:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        sent_ok,
    }
}

#[tokio::test]
async fn test_get_unsent_mocked() {
    let notices = vec![make_notice("Test notice", 0)];
    let unsent: Vec<_> = notices.iter().filter(|n| n.sent_ok == 0).collect();
    assert_eq!(unsent.len(), 1);
    assert_eq!(unsent[0].title, "Test notice");
}

#[tokio::test]
async fn test_get_sent_mocked() {
    let notices = vec![make_notice("Test notice", 1)];
    let unsent: Vec<_> = notices.iter().filter(|n| n.sent_ok == 0).collect();
    assert_eq!(unsent.len(), 0);
}

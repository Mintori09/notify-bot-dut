use async_trait::async_trait;
use notify_bot_dut::entities::notice_sent;

#[async_trait]
pub trait NoticeRepo {
    async fn get_unsent(&self) -> anyhow::Result<Vec<notice_sent::Model>>;
}

pub struct NoticeRepoMock {
    pub data: Vec<notice_sent::Model>,
}

#[async_trait]
impl NoticeRepo for NoticeRepoMock {
    async fn get_unsent(&self) -> anyhow::Result<Vec<notice_sent::Model>> {
        // filter out records where sent_ok != 0
        Ok(self
            .data
            .iter()
            .cloned()
            .filter(|m| m.sent_ok == 0)
            .collect())
    }
}

#[tokio::test]
async fn test_get_unsent_mocked() {
    let mock_data = vec![notice_sent::Model {
        id: 1,
        main_category: "ClassNotice".into(),
        external_id: "abc123".into(),
        published_date: None,
        title: "Test notice".into(),
        body: Some("Body content".into()),
        sent_at: chrono::NaiveDateTime::parse_from_str("2025-09-01 12:30:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        sent_ok: 0,
    }];

    let repo = NoticeRepoMock { data: mock_data };

    let unsent = repo.get_unsent().await.unwrap();
    assert_eq!(unsent.len(), 1);
    assert_eq!(unsent[0].title, "Test notice");
}

#[tokio::test]
async fn test_get_sent_mocked() {
    let mock_data = vec![notice_sent::Model {
        id: 1,
        main_category: "ClassNotice".into(),
        external_id: "abc123".into(),
        published_date: None,
        title: "Test notice".into(),
        body: Some("Body content".into()),
        sent_at: chrono::NaiveDateTime::parse_from_str("2025-09-01 12:30:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        sent_ok: 1,
    }];

    let repo = NoticeRepoMock { data: mock_data };

    let unsent = repo.get_unsent().await.unwrap();
    assert_eq!(unsent.len(), 0);
}

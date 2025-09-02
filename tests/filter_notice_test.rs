use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use notify_bot_dut::database::Config;
use notify_bot_dut::entity::{Category, NoticeSent};
use notify_bot_dut::utils::filter_notice;

fn make_notice(category: Category, title: String) -> NoticeSent {
    NoticeSent {
        id: 1,
        external_id: "ext-1".to_string(),
        published_date: Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        title: title,
        body: Some("sdjlsdljf".into()),
        main_category: category,
        sent_at: NaiveDateTime::from_timestamp_opt(1_700_000_000, 0).unwrap(),
        sent_ok: 0,
    }
}

fn make_config(filters: Option<Vec<&str>>) -> Config {
    Config {
        database_url: "sqlite::memory:".to_string(),
        teloxide_token: "dummy".to_string(),
        chat_id: 12345,
        filter: filters.map(|fs| fs.into_iter().map(|s| s.to_string()).collect()),
    }
}

#[tokio::test]
async fn test_non_class_notice_always_true() -> Result<()> {
    let notice = make_notice(Category::Tuition, "Any content".into());
    let config = make_config(Some(vec!["abc"]));

    let result = filter_notice(&notice, &config).await?;
    assert_eq!(result, true); // vì không phải ClassNotice → luôn true
    Ok(())
}

#[tokio::test]
async fn test_class_notice_with_matching_filter() -> Result<()> {
    let notice = make_notice(
        Category::ClassNotice,
        "Thầy Trương Ngọc Sơn thông báo đến lớp: Thẩm định dự án nâng cao [21.Nh84]".into(),
    );
    let config = make_config(Some(vec!["21.Nh84"]));

    let result = filter_notice(&notice, &config).await?;
    assert_eq!(result, true); // có từ khóa 
    Ok(())
}

#[tokio::test]
async fn test_class_notice_without_matching_filter() -> Result<()> {
    let notice = make_notice(Category::ClassNotice, "Thông báo môn Toán".into());
    let config = make_config(Some(vec!["Hóa"]));

    let result = filter_notice(&notice, &config).await?;
    assert_eq!(result, false); // không chứa "Hóa"
    Ok(())
}

#[tokio::test]
async fn test_class_notice_with_no_filter() -> Result<()> {
    let notice = make_notice(Category::ClassNotice, "Thông báo bất kỳ".into());
    let config = make_config(None);

    let result = filter_notice(&notice, &config).await?;
    assert_eq!(result, true); // không có filter → mặc định true
    Ok(())
}

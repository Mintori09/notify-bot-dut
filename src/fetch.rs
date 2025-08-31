use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::entity::NoticeSent;

pub fn http_client() -> Client {
    Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                     AppleWebKit/537.36 (KHTML, like Gecko) \
                     Chrome/117.0.0.0 Safari/537.36",
        )
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .expect("Failed to create reqwest client")
}

pub fn analysis_notice(html: &str) -> Result<Vec<NoticeSent>> {
    let fragment = Html::parse_document(html);

    let box_sel = Selector::parse("div.tbBox").unwrap();
    let caption_sel = Selector::parse("div.tbBoxCaption").unwrap();
    let content_sel = Selector::parse("div.tbBoxContent").unwrap();

    let mut results = Vec::new();

    for tb in fragment.select(&box_sel) {
        let caption = tb.select(&caption_sel).next();
        let content = tb.select(&content_sel).next();

        let caption_text = caption
            .map(|c| c.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .unwrap_or_default();

        let content_text = content
            .map(|c| c.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .unwrap_or_default();

        let (date, title) = if let Some((d, t)) = caption_text.split_once(':') {
            (d.trim().to_string(), t.trim().to_string())
        } else {
            ("".to_string(), caption_text)
        };

        results.push(NoticeSent::new(date, title, content_text));
    }

    Ok(results)
}

/// Training
pub async fn fetch_training(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBSV&PAGETB=1&COL=TieuDe&NAME=&TAB=0";
    let body = client.get(url).send().await?.text().await?;
    analysis_notice(&body)
}

/// Class notices
pub async fn fetch_class_notice(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBGV&PAGETB=1&COL=TieuDe&NAME=&TAB=1";
    let body = client.get(url).send().await?.text().await?;
    analysis_notice(&body)
}

/// Student affairs
pub async fn fetch_student_affairs(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBSV&PAGETB=1&COL=TieuDe&NAME=&TAB=2";
    let body = client.get(url).send().await?.text().await?;
    analysis_notice(&body)
}

/// Tuition
pub async fn fetch_tuition(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBSV&PAGETB=1&COL=TieuDe&NAME=&TAB=4";
    let body = client.get(url).send().await?.text().await?;
    analysis_notice(&body)
}

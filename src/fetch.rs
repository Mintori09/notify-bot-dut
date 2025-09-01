use std::{thread::sleep, time::Duration};

use anyhow::Result;
use reqwest::Client;
use scraper::{ElementRef, Html, Node, Selector};

use crate::entity::{Category, NoticeSent};

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

fn collect_text_with_links(el: &ElementRef) -> String {
    let mut buf = String::new();
    for child in el.children() {
        match child.value() {
            Node::Text(t) => {
                buf.push_str(t);
            }
            Node::Element(e) if e.name() == "a" => {
                if let Some(href) = e.attr("href") {
                    let link_text = ElementRef::wrap(child)
                        .map(|a| a.text().collect::<Vec<_>>().join(" "))
                        .unwrap_or_default();
                    buf.push_str(&format!(r#"<a href="{href}">{}</a>"#, link_text.trim()));
                }
            }
            Node::Element(_) => {
                if let Some(child_el) = ElementRef::wrap(child) {
                    buf.push_str(&collect_text_with_links(&child_el));
                }
            }
            _ => {}
        }
    }
    buf.trim().to_string()
}

pub fn analysis_notice(html: &str, category: Category) -> Result<Vec<NoticeSent>> {
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
            .map(|c| collect_text_with_links(&c))
            .unwrap_or_default();

        let (date, title) = if let Some((d, t)) = caption_text.split_once(':') {
            (Some(d.trim().to_string()), t.trim().to_string())
        } else {
            (None, caption_text)
        };

        results.push(NoticeSent::new(
            date,
            title,
            if content_text.is_empty() {
                None
            } else {
                Some(content_text)
            },
            category.clone(),
        ));
    }

    Ok(results)
}

pub async fn fetch_category(
    client: &Client,
    url: &str,
    category: Category,
) -> Result<Vec<NoticeSent>> {
    loop {
        println!("Fetching data for category: {category}");

        match client.get(url).send().await {
            Ok(resp) => match resp.text().await {
                Ok(body) => match analysis_notice(&body, category.clone()) {
                    Ok(notices) => {
                        println!("Success: {} notices found for {category}", notices.len());
                        return Ok(notices);
                    }
                    Err(err) => eprintln!("Parse error for {category}: {err}"),
                },
                Err(err) => eprintln!("Failed to read response body for {category}: {err}"),
            },
            Err(err) => eprintln!("HTTP request failed for {category}: {err}"),
        }

        println!("⏳ Retrying {category} fetch in 10s...");
        sleep(Duration::from_secs(10));
    }
}

pub async fn fetch_training(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBSV&PAGETB=1&COL=TieuDe&NAME=&TAB=0";
    fetch_category(client, url, Category::Training).await
}

pub async fn fetch_class_notice(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBGV&PAGETB=1&COL=TieuDe&NAME=&TAB=1";
    fetch_category(client, url, Category::ClassNotice).await
}

pub async fn fetch_student_affairs(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBSV&PAGETB=1&COL=TieuDe&NAME=&TAB=2";
    fetch_category(client, url, Category::StudentAffairs).await
}

pub async fn fetch_tuition(client: &Client) -> Result<Vec<NoticeSent>> {
    let url =
        "https://sv.dut.udn.vn/WebAjax/evLopHP_Load.aspx?E=CTRTBSV&PAGETB=1&COL=TieuDe&NAME=&TAB=4";
    fetch_category(client, url, Category::Tuition).await
}

pub async fn fetch_all_notices(client: &reqwest::Client) -> Result<Vec<NoticeSent>> {
    let mut all = Vec::new();

    all.extend(fetch_training(client).await?);
    all.extend(fetch_class_notice(client).await?);
    all.extend(fetch_student_affairs(client).await?);
    all.extend(fetch_tuition(client).await?);

    all.sort_by(|a, b| a.published_date.cmp(&b.published_date));

    Ok(all)
}

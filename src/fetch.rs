use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

pub fn analysis(html: &str, selector: &str, prefix: &str) -> Result<Vec<String>> {
    let fragment = Html::parse_document(html);
    let selector =
        Selector::parse(selector).map_err(|_| anyhow::anyhow!("Invalid selector: {selector}"))?;

    let mut results = Vec::new();
    for element in fragment.select(&selector) {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text.is_empty() {
            results.push(format!("{prefix}: {text}"));
        }
    }

    Ok(results)
}

pub async fn fetch_training(client: &Client) -> Result<Vec<String>> {
    let url = "https://example.com/training";
    let body = client.get(url).send().await?.text().await?;
    analysis(&body, "a.notice-link", "Training")
}

pub async fn fetch_class_notice(client: &Client) -> Result<Vec<String>> {
    let url = "https://example.com/class-notice";
    let body = client.get(url).send().await?.text().await?;
    analysis(&body, "div.class-notice", "Class")
}

pub async fn fetch_student_affairs(client: &Client) -> Result<Vec<String>> {
    let url = "https://example.com/student-affairs";
    let body = client.get(url).send().await?.text().await?;
    analysis(&body, "li.item", "Student Affairs")
}

pub async fn fetch_tuition(client: &Client) -> Result<Vec<String>> {
    let url = "https://example.com/tuition";
    let body = client.get(url).send().await?.text().await?;
    analysis(&body, "p.fee", "Tuition")
}

/// Aggregate all
pub async fn fetch_notices(client: &Client) -> Result<Vec<String>> {
    let mut all = Vec::new();
    all.extend(fetch_training(client).await?);
    all.extend(fetch_class_notice(client).await?);
    all.extend(fetch_student_affairs(client).await?);
    all.extend(fetch_tuition(client).await?);
    Ok(all)
}

/// Shared HTTP client
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

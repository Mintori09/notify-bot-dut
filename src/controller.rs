use crate::entity::{self, NoticeSent};
use crate::utils::filter_notice;
use crate::database::Config;
use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::SqlitePool;

fn parse_sent_at(s: &str) -> anyhow::Result<NaiveDateTime> {
    // Full datetime stored by CURRENT_TIMESTAMP
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Ok(dt);
    }
    // Date-only stored by old CURRENT_DATE default
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        .map_err(|_| anyhow::anyhow!("Invalid sent_at: {s}"))
}

pub async fn check_and_insert(
    db: &SqlitePool,
    notice: &entity::NoticeSent,
    config: &Config,
) -> Result<bool> {
    if !filter_notice(notice, config).await? {
        println!("Skipped by filter: {}", notice.title);
        return Ok(false);
    }

    let exists: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM notice_sent WHERE main_category = ? AND external_id = ? LIMIT 1",
    )
    .bind(notice.main_category.to_string())
    .bind(&notice.external_id)
    .fetch_optional(db)
    .await?;

    if exists.is_some() {
        return Ok(false);
    }

    sqlx::query(
        "INSERT INTO notice_sent (main_category, external_id, published_date, title, body, sent_ok)
         VALUES (?, ?, ?, ?, ?, 0)",
    )
    .bind(notice.main_category.to_string())
    .bind(&notice.external_id)
    .bind(notice.published_date)
    .bind(&notice.title)
    .bind(&notice.body)
    .execute(db)
    .await?;

    Ok(true)
}

pub async fn mark_as_sent(db: &SqlitePool, notice: &entity::NoticeSent) -> Result<()> {
    sqlx::query(
        "UPDATE notice_sent SET sent_ok = 1 WHERE main_category = ? AND external_id = ?",
    )
    .bind(notice.main_category.to_string())
    .bind(&notice.external_id)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_unsent(db: &SqlitePool) -> Result<Vec<entity::NoticeSent>> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        main_category: String,
        external_id: String,
        published_date: Option<NaiveDate>,
        title: String,
        body: Option<String>,
        sent_at: String,
        sent_ok: i64,
    }

    let rows = sqlx::query_as::<_, Row>(
        "SELECT id, main_category, external_id, published_date, title, body, sent_at, sent_ok
         FROM notice_sent WHERE sent_ok = 0",
    )
    .fetch_all(db)
    .await?;

    rows.into_iter()
        .map(|r| {
            Ok(NoticeSent {
                id: r.id as i32,
                main_category: r.main_category.parse().map_err(anyhow::Error::msg)?,
                external_id: r.external_id,
                published_date: r.published_date,
                title: r.title,
                body: r.body,
                sent_at: parse_sent_at(&r.sent_at)?,
                sent_ok: r.sent_ok as i32,
            })
        })
        .collect()
}

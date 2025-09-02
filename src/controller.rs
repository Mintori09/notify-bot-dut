use crate::entity::{self, NoticeSent};
use crate::utils::filter_notice;
use crate::{database::Config, entities::notice_sent};
use anyhow::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn check_and_insert(
    db: &sea_orm::DatabaseConnection,
    notice: &entity::NoticeSent,
    config: &Config,
) -> Result<bool> {
    if !filter_notice(notice, config).await? {
        println!("Skipped by filter: {}", notice.title);
        return Ok(false);
    }

    let exists = notice_sent::Entity::find()
        .filter(notice_sent::Column::MainCategory.eq(notice.main_category.to_string()))
        .filter(notice_sent::Column::ExternalId.eq(notice.external_id.clone()))
        .one(db)
        .await?;

    if exists.is_some() {
        return Ok(false);
    }

    let new_row = notice_sent::ActiveModel {
        main_category: Set(notice.main_category.to_string()),
        external_id: Set(notice.external_id.clone()),
        published_date: Set(notice.published_date),
        title: Set(notice.title.clone()),
        body: Set(notice.body.clone()),
        sent_ok: Set(0),
        ..Default::default()
    };

    new_row.insert(db).await?;
    Ok(true)
}

pub async fn mark_as_sent(
    db: &sea_orm::DatabaseConnection,
    notice: &entity::NoticeSent,
) -> Result<()> {
    if let Some(found) = notice_sent::Entity::find()
        .filter(notice_sent::Column::MainCategory.eq(notice.main_category.to_string()))
        .filter(notice_sent::Column::ExternalId.eq(notice.external_id.clone()))
        .one(db)
        .await?
    {
        let mut active: notice_sent::ActiveModel = found.into();
        active.sent_ok = Set(1);
        active.update(db).await?;
    }
    Ok(())
}

pub async fn get_unsent(db: &sea_orm::DatabaseConnection) -> Result<Vec<entity::NoticeSent>> {
    let rows = notice_sent::Entity::find()
        .filter(notice_sent::Column::SentOk.eq(0))
        .all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(NoticeSent::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

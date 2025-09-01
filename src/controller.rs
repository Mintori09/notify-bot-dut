use crate::entities::notice_sent;
use crate::entity;
use anyhow::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn check_and_insert(
    db: &sea_orm::DatabaseConnection,
    notice: &entity::NoticeSent,
) -> Result<bool> {
    // 1. Check existence
    let exists = notice_sent::Entity::find()
        .filter(notice_sent::Column::MainCategory.eq(notice.main_category.to_string()))
        .filter(notice_sent::Column::ExternalId.eq(notice.external_id.clone()))
        .one(db)
        .await?;

    if exists.is_some() {
        return Ok(false);
    }

    // 2. Insert new
    let new_row = notice_sent::ActiveModel {
        main_category: Set(notice.main_category.to_string()),
        external_id: Set(notice.external_id.clone()),
        published_date: Set(notice.published_date), // ✅ type aligned
        title: Set(notice.title.clone()),
        body: Set(notice.body.clone()), // ✅ already Option<String>
        ..Default::default()
    };

    new_row.insert(db).await?;
    Ok(true)
}

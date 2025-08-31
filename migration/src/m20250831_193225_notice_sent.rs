use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoticeSent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NoticeSent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(NoticeSent::MainCategory)
                            .text()
                            .not_null()
                            .check("main_category IN ('Training', 'ClassNotice', 'StudentAffairs', 'Tuition')"),
                    )
                    .col(
                        ColumnDef::new(NoticeSent::ExternalId)
                            .text()
                            .not_null()
                            .check("length(external_id) > 0"),
                    )
                    .col(
                        ColumnDef::new(NoticeSent::PublishedDate)
                            .text()
                            .null()
                            .check("published_date IS NULL OR published_date GLOB '[0-9][0-9][0-9][0-9]-[0-1][0-9]-[0-3][0-9]'"),
                    )
                    .col(
                        ColumnDef::new(NoticeSent::Body)
                            .text()
                            .null()
                            .check("body IS NULL OR length(trim(body)) > 0"),
                    )
                    .col(
                        ColumnDef::new(NoticeSent::Title)
                            .text()
                            .not_null()
                            .check("length(trim(title)) >= 3"),
                    )
                    .col(
                        ColumnDef::new(NoticeSent::SentAt)
                            .text()
                            .not_null()
                            .default(Expr::cust("datetime('now')"))
                            .check("sent_at GLOB '____-__-__ __:__:__'"),
                    )
                    .index(
                        Index::create()
                            .name("unique_main_category_external_id")
                            .col(NoticeSent::MainCategory)
                            .col(NoticeSent::ExternalId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NoticeSent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NoticeSent {
    Table,
    Id,
    MainCategory,
    ExternalId,
    PublishedDate,
    Body,
    Title,
    SentAt,
}

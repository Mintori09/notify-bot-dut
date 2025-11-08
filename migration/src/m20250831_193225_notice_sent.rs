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
                    .col(ColumnDef::new(NoticeSent::MainCategory).text().not_null())
                    .col(ColumnDef::new(NoticeSent::ExternalId).text().not_null())
                    .col(ColumnDef::new(NoticeSent::PublishedDate).date().null())
                    .col(ColumnDef::new(NoticeSent::Body).text().null())
                    .col(ColumnDef::new(NoticeSent::Title).text().not_null())
                    .col(
                        ColumnDef::new(NoticeSent::SentAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_DATE")),
                    )
                    .col(
                        ColumnDef::new(NoticeSent::SentOk)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .check(Expr::cust(
                        "(main_category IN ('Training','ClassNotice','StudentAffairs','Tuition'))",
                    ))
                    .check(Expr::cust("(length(external_id) > 0)"))
                    .check(Expr::cust("(body IS NULL OR length(trim(body)) > 0)"))
                    .check(Expr::cust("(length(trim(title)) >= 3)"))
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
    #[sea_orm(iden = "notice_sent")]
    Table,
    Id,
    MainCategory,
    ExternalId,
    PublishedDate,
    Body,
    Title,
    SentAt,
    SentOk,
}

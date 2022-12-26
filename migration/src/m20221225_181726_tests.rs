use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Tests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tests::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tests::Title).string().not_null())
                    .col(ColumnDef::new(Tests::Description).string().not_null())
                    .col(ColumnDef::new(Tests::AuthorId).integer().not_null())
                    .col(ColumnDef::new(Tests::Deleted).boolean())
                    .col(ColumnDef::new(Tests::Created).date().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Tests::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Tests {
    Table,
    Id,
    Title,
    Description,
    AuthorId,
    Deleted,
    Created
}

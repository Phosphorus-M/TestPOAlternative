use sea_orm_migration::prelude::*;

use crate::{m20221225_181726_tests::Tests, m20230602_091749_questions::Questions};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
            .table(Questions::Table)
            .add_column(
                ColumnDef::new(Questions::TestId)
                .integer()
                .not_null()
            )
            .to_owned()
        ).await?;

        manager.create_foreign_key(
            ForeignKey::create()
            .name("fk_test_id")
            .from(Questions::Table, Questions::TestId)
            .to(Tests::Table, Tests::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
            .table(Questions::Table)
            .drop_column(Questions::TestId)
            .to_owned()
        ).await?;

        manager.drop_foreign_key(
            ForeignKey::drop()
            .name("fk_test_id")
            .table(Questions::Table)
            .to_owned()
        ).await?;

        Ok(())
    }
}

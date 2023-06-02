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
                    .table(Questions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Questions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Questions::Text).string().not_null())
                    .col(ColumnDef::new(Questions::Answers).integer().not_null())
                    .col(ColumnDef::new(Questions::CorrectAnswer).integer().not_null())
                    .col(ColumnDef::new(Questions::Deleted).boolean())
                    .col(ColumnDef::new(Questions::Created).date().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Answers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Answers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Answers::Text).string().not_null())
                    .col(ColumnDef::new(Answers::QuestionId).integer().not_null())
                    .col(ColumnDef::new(Answers::Deleted).boolean())
                    .col(ColumnDef::new(Answers::Created).date().extra("DEFAULT CURRENT_TIMESTAMP".to_string()))
                    .to_owned(),
            ).await?;

            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_question_id")
                    .from(Answers::Table, Answers::QuestionId)
                    .to(Questions::Table, Questions::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            ).await?;

            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_correct_answer_id")
                    .from(Questions::Table, Questions::CorrectAnswer)
                    .to(Answers::Table, Answers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Questions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Answers::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Questions {
    Table,
    Id,
    Text,
    Answers,
    CorrectAnswer,
    TestId,
    Deleted,
    Created
}

#[derive(Iden)]
enum Answers {
    Table,
    Id,
    Text,
    QuestionId,
    Deleted,
    Created
}
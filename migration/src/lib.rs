pub use sea_orm_migration::prelude::*;

mod m20221225_181726_tests;
mod m20230602_091749_questions;
mod m20230602_100831_add_questions_in_test;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221225_181726_tests::Migration),
            Box::new(m20230602_091749_questions::Migration),
            Box::new(m20230602_100831_add_questions_in_test::Migration),
        ]
    }
}

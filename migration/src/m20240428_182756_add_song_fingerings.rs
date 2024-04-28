use sea_orm_migration::{prelude::*, schema::*};

use crate::tables::Song;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Song::Table)
                    .add_column(string_null(Song::Fingerings))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Song::Table)
                    .drop_column(Song::Fingerings)
                    .to_owned(),
            )
            .await
    }
}

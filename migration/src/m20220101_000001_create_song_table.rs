use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Song::Table)
                    .col(string(Song::Id).primary_key())
                    .col(string(Song::Author))
                    .col(string(Song::Title))
                    .col(string(Song::Tablature))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Song::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Song {
    Table,
    Id,
    Author,
    Title,
    Tablature,
}

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const SONG_OWNER_INDEX_NAME: &str = "idx_song_owner";
const USER_EMAIL_INDEX_NAME: &str = "idx_user_email";
const USER_NAME_INDEX_NAME: &str = "idx_user_name";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Song::Table)
                    .col(string(Song::Id).primary_key())
                    .col(string(Song::Owner))
                    .col(string(Song::Author))
                    .col(string(Song::Title))
                    .col(string(Song::Tablature))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name(SONG_OWNER_INDEX_NAME)
                    .table(Song::Table)
                    .col(Song::Owner)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(string(User::Id).primary_key())
                    .col(string(User::Name))
                    .col(string(User::Email))
                    .col(string(User::Password))
                    .col(boolean(User::IsAdmin))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name(USER_EMAIL_INDEX_NAME)
                    .table(User::Table)
                    .col(User::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                IndexCreateStatement::new()
                    .name(USER_NAME_INDEX_NAME)
                    .table(User::Table)
                    .col(User::Name)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name(SONG_OWNER_INDEX_NAME).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Song::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Song {
    Table,
    Id,
    Owner,
    Author,
    Title,
    Tablature,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    IsAdmin,
}

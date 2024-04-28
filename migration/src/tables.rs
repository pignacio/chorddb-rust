use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub(crate) enum Song {
    Table,
    Id,
    Author,
    Title,
    Tablature,
    Fingerings,
}

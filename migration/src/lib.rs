pub use sea_orm_migration::prelude::*;

mod tables;

mod m20220101_000001_create_song_table;
mod m20240428_182756_add_song_fingerings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_song_table::Migration),
            Box::new(m20240428_182756_add_song_fingerings::Migration),
        ]
    }
}

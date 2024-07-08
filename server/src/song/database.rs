use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait, Iterable};
use uuid::Uuid;

use crate::{
    entities::prelude::Song as SongEntity,
    entities::song,
    error::{ChordDbError, ChordDbResult},
};

use super::{Song, SongHeader};

pub struct SeaOrmSongs {
    db: DatabaseConnection,
}

fn build_header(model: &song::Model) -> ChordDbResult<SongHeader> {
    Uuid::parse_str(&model.id)
        .map_err(|err| {
            ChordDbError::InvalidData(format!("Invalid uuid: '{}'. Err: {}", model.id, err))
        })
        .map(|id| SongHeader {
            id,
            author: model.author.clone(),
            title: model.title.clone(),
        })
}

impl SeaOrmSongs {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn all_songs(&self) -> ChordDbResult<Vec<SongHeader>> {
        let entities = SongEntity::find().all(&self.db).await?;
        entities.iter().map(build_header).collect()
    }

    pub async fn upsert_song(&self, song: Song) -> ChordDbResult<()> {
        let model = song::Model {
            id: song.id().to_string(),
            author: song.author().to_string(),
            title: song.title().to_string(),
            tablature: song.contents,
            owner: "8e4ca15e-42cf-4479-b45c-b2815c679cb2".to_string(),
        };

        SongEntity::insert(song::ActiveModel::from(model))
            .on_conflict(
                OnConflict::column(song::Column::Id)
                    .update_columns(song::Column::iter())
                    .to_owned(),
            )
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn get_song(&self, id: &Uuid) -> ChordDbResult<Option<Song>> {
        let song = SongEntity::find_by_id(*id).one(&self.db).await?;
        Ok(match song {
            Some(model) => Some(Song {
                header: build_header(&model)?,
                contents: model.tablature,
            }),
            None => None,
        })
    }

    pub async fn delete_song(&self, id: &Uuid) -> ChordDbResult<()> {
        SongEntity::delete_by_id(*id).exec(&self.db).await?;
        Ok(())
    }
}

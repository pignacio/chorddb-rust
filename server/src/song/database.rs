use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::error::{ChordDbError, ChordDbResult};

use super::{Song, SongHeader};

pub struct SeaOrmSongs {
    db: DatabaseConnection,
}

fn build_header(model: &crate::entities::song::Model) -> ChordDbResult<SongHeader> {
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
        let entities = crate::entities::prelude::Song::find().all(&self.db).await?;
        entities.iter().map(build_header).collect()
    }

    pub async fn add_song(&self, song: Song) -> ChordDbResult<()> {
        let model = crate::entities::song::Model {
            id: song.id().to_string(),
            author: song.author().to_string(),
            title: song.title().to_string(),
            tablature: song.contents,
        };

        crate::entities::prelude::Song::insert(crate::entities::song::ActiveModel::from(model))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn get_song(&self, id: &Uuid) -> ChordDbResult<Option<Song>> {
        let song = crate::entities::prelude::Song::find_by_id(*id)
            .one(&self.db)
            .await?;
        Ok(match song {
            Some(model) => Some(Song {
                header: build_header(&model)?,
                contents: model.tablature,
            }),
            None => None,
        })
    }
}

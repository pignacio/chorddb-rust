use axum::async_trait;
use sea_orm::{sea_query::OnConflict, EntityTrait, Iterable};

use crate::{entities::session, error::ChordDbResult};

pub use crate::entities::session::{Entity as SessionEntity, Model as Session};

#[async_trait]
pub trait Sessions: Send + Sync {
    async fn get_session(&self, session_id: &str) -> ChordDbResult<Option<Session>>;
    async fn upsert_session(&self, session: Session) -> ChordDbResult<()>;
}

pub struct SeaOrmSessions {
    db: sea_orm::DatabaseConnection,
}

impl SeaOrmSessions {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Sessions for SeaOrmSessions {
    async fn get_session(&self, session_id: &str) -> ChordDbResult<Option<Session>> {
        let session = SessionEntity::find_by_id(session_id).one(&self.db).await?;
        Ok(session)
    }

    async fn upsert_session(&self, session: Session) -> ChordDbResult<()> {
        SessionEntity::insert(session::ActiveModel::from(session))
            .on_conflict(
                OnConflict::column(session::Column::Id)
                    .update_columns(session::Column::iter())
                    .to_owned(),
            )
            .exec(&self.db)
            .await?;
        Ok(())
    }
}

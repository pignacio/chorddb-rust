use axum::async_trait;
use uuid::Uuid;

use crate::error::ChordDbResult;

mod database;

pub use database::SeaOrmUsers;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait Users: Send + Sync {
    async fn get_user(&self, id: &str) -> ChordDbResult<Option<User>>;
}

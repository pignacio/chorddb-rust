use axum::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    entities::{prelude::User as UserEntity, user},
    error::ChordDbResult,
};

use super::{User, Users};

pub struct SeaOrmUsers {
    db: DatabaseConnection,
}

impl SeaOrmUsers {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn get_user_by_id(&self, id: &str) -> ChordDbResult<Option<user::Model>> {
        if let Some(user) = UserEntity::find_by_id(id).one(&self.db).await? {
            Ok(Some(user))
        } else if let Some(user) = UserEntity::find()
            .filter(user::Column::Name.eq(id))
            .one(&self.db)
            .await?
        {
            Ok(Some(user))
        } else {
            Ok(UserEntity::find()
                .filter(user::Column::Email.eq(id))
                .one(&self.db)
                .await?)
        }
    }
}

#[async_trait]
impl Users for SeaOrmUsers {
    async fn get_user(&self, id: &str) -> ChordDbResult<Option<User>> {
        let entity = self.get_user_by_id(id).await?;

        Ok(entity.map(|model| User {
            id: Uuid::parse_str(&model.id).unwrap(),
            name: model.name,
            email: model.email,
            password: model.password,
            is_admin: model.is_admin,
        }))
    }
}

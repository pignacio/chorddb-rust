use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;

#[derive(Debug)]
pub enum ChordDbError {
    HttpNotFound,
    Database(DbErr),
    InvalidData(String),
}

impl Display for ChordDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ChordDbError::Database(e) => format!("Database: {}", e),
            ChordDbError::HttpNotFound => "HttpNotFound".to_string(),
            ChordDbError::InvalidData(msg) => format!("InvalidData: {}", msg),
        };
        f.write_fmt(format_args!("ChordDbError::{}", message))
    }
}

impl Error for ChordDbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChordDbError::Database(e) => Some(e),
            ChordDbError::HttpNotFound => None,
            ChordDbError::InvalidData(_) => None,
        }
    }
}

impl From<DbErr> for ChordDbError {
    fn from(value: DbErr) -> Self {
        Self::Database(value)
    }
}

impl IntoResponse for ChordDbError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ChordDbError::HttpNotFound => StatusCode::NOT_FOUND.into_response(),
            _ => {
                log::warn!("Request failed! {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong :(").into_response()
            }
        }
    }
}

pub type ChordDbResult<T> = Result<T, ChordDbError>;
use std::{error::Error, fmt::Display};

use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug)]
pub enum ChordDbError {
    HttpNotFound,
    Database(DbErr),
    InvalidData(String),
    BadRequest(String),
    Generic(Box<dyn Error>),
}

impl Display for ChordDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ChordDbError::Database(e) => format!("Database: {}", e),
            ChordDbError::HttpNotFound => "HttpNotFound".to_string(),
            ChordDbError::InvalidData(msg) => format!("InvalidData: {}", msg),
            ChordDbError::BadRequest(msg) => format!("BadRequest: {}", msg),
            ChordDbError::Generic(e) => format!("Generic: {}", e),
        };
        f.write_fmt(format_args!("ChordDbError::{}", message))
    }
}

impl Error for ChordDbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChordDbError::Database(e) => Some(e),
            ChordDbError::Generic(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<DbErr> for ChordDbError {
    fn from(value: DbErr) -> Self {
        Self::Database(value)
    }
}

#[derive(Debug, Serialize)]
struct SimpleError {
    message: String,
}

impl SimpleError {
    fn new<S: AsRef<str>>(message: S) -> Self {
        Self {
            message: message.as_ref().to_string(),
        }
    }
}

impl IntoResponse for ChordDbError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ChordDbError::HttpNotFound => (
                StatusCode::NOT_FOUND,
                Json(SimpleError::new("404 Not Found")),
            )
                .into_response(),
            ChordDbError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, Json(SimpleError::new(message))).into_response()
            }
            _ => {
                log::warn!("Request failed! {}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong :(").into_response()
            }
        }
    }
}

pub type ChordDbResult<T> = Result<T, ChordDbError>;

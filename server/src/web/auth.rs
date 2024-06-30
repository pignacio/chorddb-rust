use axum::{extract::Query, response::Redirect, Form, Json};
use serde::{Deserialize, Serialize};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::error::{ChordDbError, ChordDbResult};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub logged_in: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

impl UserData {
    fn anonymous() -> Self {
        Self {
            logged_in: false,
            email: None,
        }
    }

    fn user(email: String) -> Self {
        Self {
            logged_in: true,
            email: Some(email),
        }
    }
}

pub fn get_user_data(cookies: &Cookies) -> UserData {
    cookies
        .get("token")
        .map(|token| UserData::user(token.value().to_string()))
        .unwrap_or(UserData::anonymous())
}

pub async fn user_data(cookies: Cookies) -> ChordDbResult<Json<UserData>> {
    Ok(Json(get_user_data(&cookies)))
}

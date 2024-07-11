use axum::{extract::State, Json};
use axum_macros::debug_handler;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tower_cookies::{
    cookie::{time::Duration, SameSite},
    Cookie, Cookies,
};
use uuid::Uuid;

use crate::{
    error::{ChordDbError, ChordDbResult},
    session::{Session, Sessions},
    user::Users,
};

use super::AppState;

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

const SESSION_COOKIE_NAME: &str = "session_chorddb";

pub async fn get_user_data(
    cookies: &Cookies,
    users: &dyn Users,
    sessions: &dyn Sessions,
) -> ChordDbResult<UserData> {
    Ok(maybe_user_data(cookies, users, sessions)
        .await?
        .unwrap_or_else(UserData::anonymous))
}

pub async fn maybe_user_data(
    cookies: &Cookies,
    users: &dyn Users,
    sessions: &dyn Sessions,
) -> ChordDbResult<Option<UserData>> {
    let Some(cookie) = cookies.get(SESSION_COOKIE_NAME) else {
        log::debug!("no cookie");
        return Ok(None);
    };
    let Some(session) = sessions.get_session(cookie.value()).await? else {
        log::debug!("no session for cookie {:?}", cookie);
        return Ok(None);
    };
    let Some(user) = users.get_user(&session.user_id).await? else {
        log::debug!("no user for id {}", session.user_id);
        return Ok(None);
    };
    Ok(Some(UserData::user(user.email)))
}

#[debug_handler]
pub(super) async fn user_data(
    State(AppState {
        users, sessions, ..
    }): State<AppState>,
    cookies: Cookies,
) -> ChordDbResult<Json<UserData>> {
    Ok(Json(
        get_user_data(&cookies, users.as_ref(), sessions.as_ref()).await?,
    ))
}

#[derive(Debug, Deserialize)]
pub(super) struct LoginPayload {
    pub user: String,
    pub password: String,
}

pub(super) async fn login(
    State(AppState {
        users, sessions, ..
    }): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> ChordDbResult<Json<UserData>> {
    let Some(user) = users.get_user(&payload.user).await? else {
        return Err(ChordDbError::BadRequest("Invalid user".to_string()));
    };

    if !verify_password(&payload.password, &user.password) {
        return Err(ChordDbError::BadRequest("Invalid password".to_string()));
    }

    let session = Uuid::new_v4().to_string();
    sessions
        .upsert_session(Session {
            id: session.clone(),
            user_id: user.id.to_string(),
            expires_at: (Utc::now() + chrono::Duration::days(7)).naive_utc(),
        })
        .await?;

    cookies.add(
        Cookie::build(Cookie::new(SESSION_COOKIE_NAME, session))
            .max_age(Duration::days(7))
            .same_site(SameSite::Strict)
            .path("/")
            .secure(true)
            .http_only(true)
            .into(),
    );

    Ok(Json(UserData::user(payload.user)))
}

fn verify_password(submitted: &str, database: &str) -> bool {
    submitted == database
}

use axum::{
    extract::{Query, State},
    response::Redirect,
    Form, Json,
};
use chrono::Utc;
use jsonwebtoken_google::Parser;
use serde::{Deserialize, Serialize};
use tower_cookies::{
    cookie::{time::Duration, SameSite},
    Cookie, Cookies,
};
use uuid::Uuid;

use crate::{
    error::{ChordDbError, ChordDbResult},
    session::{Session, Sessions},
    user::{User, Users},
};

use super::AppState;

#[derive(Debug, Serialize)]
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

lazy_static! {
    static ref GOOGLE_CLIENT_ID: Option<String> = dotenv::var("GOOGLE_CLIENT_ID").ok();
}

pub async fn get_user_data(
    cookies: &Cookies,
    users: &dyn Users,
    sessions: &dyn Sessions,
) -> ChordDbResult<UserData> {
    Ok(get_authenticated_user(cookies, users, sessions)
        .await?
        .map(|user| UserData::user(user.email))
        .unwrap_or_else(UserData::anonymous))
}

pub async fn get_authenticated_user(
    cookies: &Cookies,
    users: &dyn Users,
    sessions: &dyn Sessions,
) -> ChordDbResult<Option<User>> {
    let Some(cookie) = cookies.get(SESSION_COOKIE_NAME) else {
        log::debug!("no cookie");
        return Ok(None);
    };
    let Some(session) = sessions.get_session(cookie.value()).await? else {
        log::debug!("no session for cookie {:?}", cookie);
        return Ok(None);
    };
    let user = users.get_user(&session.user_id).await?;
    if user.is_none() {
        log::debug!("no user for id {}", session.user_id);
        return Ok(None);
    };
    Ok(user)
}

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
    initialize_session(&user, sessions.as_ref(), &cookies).await?;

    Ok(Json(UserData::user(payload.user)))
}

async fn initialize_session(
    user: &User,
    sessions: &dyn Sessions,
    cookies: &Cookies,
) -> ChordDbResult<()> {
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

    Ok(())
}

fn verify_password(submitted: &str, database: &str) -> bool {
    submitted == database
}

pub(super) async fn logout(
    State(AppState { sessions, .. }): State<AppState>,
    cookies: Cookies,
) -> ChordDbResult<Redirect> {
    let Some(cookie) = cookies.get(SESSION_COOKIE_NAME) else {
        return Ok(Redirect::to("/"));
    };
    sessions.delete_session(cookie.value()).await?;
    cookies.remove(Cookie::new(SESSION_COOKIE_NAME, ""));
    Ok(Redirect::to("/"))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct GoogleLoginPayload {
    // token: String,
    g_csrf_token: String,
    credential: String,
    select_by: Option<String>,
    state: Option<String>,
    client_id: Option<String>,
    clientId: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginQueryString {
    redirect: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}

#[derive(Debug, Serialize)]
pub struct GoogleLoginResult {
    user: UserData,
}

pub async fn login_google(
    State(AppState {
        users, sessions, ..
    }): State<AppState>,
    Query(query): Query<LoginQueryString>,
    cookies: Cookies,
    Form(payload): Form<GoogleLoginPayload>,
) -> ChordDbResult<Redirect> {
    let Some(client_id) = GOOGLE_CLIENT_ID.as_ref() else {
        return Err(ChordDbError::BadRequest(
            "Google Sign In is not supported".to_string(),
        ));
    };
    let Some(crsf_cookie) = cookies.get("g_csrf_token") else {
        return Err(ChordDbError::BadRequest(
            "Missing g_csrf_token cookie".to_string(),
        ));
    };
    if crsf_cookie.value() != payload.g_csrf_token {
        return Err(ChordDbError::BadRequest(
            "Failed to verify double submit cookie".to_string(),
        ));
    }
    let parser = Parser::new(client_id);
    let decoded = parser.parse::<TokenClaims>(&payload.credential).await;
    let claims = match decoded {
        Ok(claims) => claims,
        Err(e) => {
            log::warn!("Failed to parse token: {}", e);
            return Err(ChordDbError::BadRequest("Invalid token".to_string()));
        }
    };

    // TODO: check expiration

    let Some(user) = users.get_user(&claims.email).await? else {
        return Err(ChordDbError::BadRequest(format!(
            "User {} not found",
            claims.email
        )));
    };

    initialize_session(&user, sessions.as_ref(), &cookies).await?;

    Ok(Redirect::to(&query.redirect.unwrap_or("/".to_string())))
}

use axum::{extract::Query, response::Redirect, Form, Json};
use jsonwebtoken_google::Parser;
use serde::{Deserialize, Serialize};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::error::{ChordDbError, ChordDbResult};

lazy_static! {
    static ref GOOGLE_CLIENT_ID: String =
        std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
}

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

#[derive(Deserialize)]
pub struct LoginQueryString {
    redirect: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginPayload {
    // token: String,
    g_csrf_token: String,
    credential: String,
    select_by: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}

pub async fn login(
    cookies: Cookies,
    Query(query_string): Query<LoginQueryString>,
    Form(payload): Form<LoginPayload>,
) -> ChordDbResult<Redirect> {
    let redirect_url = query_string.redirect.unwrap_or("/".to_string());
    log::info!(
        "Logging in with token. payload:{:?}, cookies:{:?}",
        payload,
        cookies
    );
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
    let parser = Parser::new(GOOGLE_CLIENT_ID.as_str());
    let claims = parser
        .parse::<TokenClaims>(&payload.credential)
        .await
        .map_err(|e| ChordDbError::Generic(Box::new(e)))?;

    log::info!("claims:{:?}", claims);

    cookies.add(
        Cookie::build(Cookie::new("token", payload.g_csrf_token.clone()))
            .path("/")
            .max_age(Duration::seconds(60 * 60 * 24))
            .build(),
    );
    return Ok(Redirect::to(&redirect_url));

    Err(ChordDbError::BadRequest(
        "Could not authenticate".to_string(),
    ))
}

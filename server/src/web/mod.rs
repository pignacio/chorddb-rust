use std::{
    collections::HashSet,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, RequestExt, Router,
};

use tower::ServiceBuilder;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use crate::{
    instrument::Instruments,
    song::{ChordRepository, SeaOrmSongs},
    Opt,
};

mod api;
mod auth;
mod chord;
mod instrument;
mod song;

#[derive(Clone)]
pub struct AppState {
    pub songs: Arc<SeaOrmSongs>,
    pub chords: Arc<dyn ChordRepository + Send + Sync>,
    pub instruments: Arc<dyn Instruments + Send + Sync>,
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

lazy_static::lazy_static! {
    static ref ANONYMOUS_URIS: HashSet<&'static str> = {
        let mut urls : HashSet<&'static str> = HashSet::new();
        urls.insert("/api/auth/user");
        urls.insert("/api/auth/login");

        urls
    };
}

async fn auth_middleware(mut request: Request, next: Next) -> impl IntoResponse {
    let cookies: Cookies = request
        .extract_parts()
        .await
        .expect("Unable to extract cookies");

    let user_data = auth::get_user_data(&cookies);
    if !user_data.logged_in && !ANONYMOUS_URIS.contains(request.uri().path()) {
        log::info!("Unauthorized request: {:?}", request.uri());
        return (StatusCode::UNAUTHORIZED, Json("Unauthorized".to_string())).into_response();
    }

    next.run(request).await
}

pub async fn run_server(opt: Opt, state: AppState) {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        // .with_same_site(tower_cookies::cookie::SameSite::Strict)
        .with_expiry(Expiry::OnInactivity(time::Duration::seconds(10)));

    let app = Router::new()
        .route("/api/auth/user", get(auth::user_data))
        .route("/api/auth/login", post(auth::login))
        .route("/api/chords/:instrument/:chord", get(chord::chords))
        .route("/api/songs", get(song::songs))
        .route("/api/songs/:id", get(song::api_song))
        .route("/api/songs/:id", patch(song::patch_song))
        .route("/api/songs/:id", delete(song::delete_song))
        .route("/api/add_song", post(song::api_add_song))
        .route("/api/instruments", get(instrument::get_instruments))
        .nest_service("/static", ServeDir::new(opt.static_dir))
        .fallback(not_found)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(middleware::from_fn(auth_middleware))
        .layer(CookieManagerLayer::new())
        // .layer(session_layer)
        .with_state(state);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .expect("Unable to start server");
}

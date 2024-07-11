use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{
    http::StatusCode,
    routing::{delete, get, patch, post},
    Router,
};

use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    instrument::Instruments,
    session::Sessions,
    song::{ChordRepository, SeaOrmSongs},
    user::Users,
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
    pub users: Arc<dyn Users>,
    pub sessions: Arc<dyn Sessions>,
    pub chords: Arc<dyn ChordRepository>,
    pub instruments: Arc<dyn Instruments>,
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

pub async fn run_server(opt: Opt, state: AppState) {
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
        .layer(CookieManagerLayer::new())
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

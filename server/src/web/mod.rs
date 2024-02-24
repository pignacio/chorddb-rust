use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use tokio::io::{stdin, AsyncBufReadExt, BufReader};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    song::{ChordRepository, SongRepository},
    Opt,
};

mod chord;
mod home;
mod not_found;
mod song;

#[derive(Clone)]
pub struct AppState {
    pub songs: Arc<dyn SongRepository + Send + Sync>,
    pub chords: Arc<dyn ChordRepository + Send + Sync>,
}

pub async fn run_server(opt: Opt, state: AppState) {
    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/", get(home::home))
        .route("/songs", post(song::add_song))
        .route("/songs/:id", get(song::song))
        .route("/api/chords/:instrument/:chord", get(chord::chords))
        .route("/api/songs", get(song::songs))
        .route("/api/songs/:id", get(song::api_song))
        .nest_service("/static", ServeDir::new(opt.static_dir))
        .fallback(not_found::not_found)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(read_stdin_until_enter())
        .await
        .expect("Unable to start server");
}

async fn read_stdin_until_enter() {
    let mut reader = BufReader::new(stdin());

    println!("Waiting for enter to stop server");
    reader.read_line(&mut String::new()).await.unwrap();

    println!("Stopping server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

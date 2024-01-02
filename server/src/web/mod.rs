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

use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{song::SongRepository, Opt};

mod home;
mod not_found;
mod song;

#[derive(Clone)]
pub struct AppState {
    pub songs: Arc<dyn SongRepository + Send + Sync>,
}

pub async fn run_server(opt: Opt, state: AppState) {
    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/", get(home::home))
        .route("/songs", post(song::add_song))
        .route("/songs/:id", get(song::song))
        .nest_service("/dist", ServeDir::new(opt.static_dir))
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
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

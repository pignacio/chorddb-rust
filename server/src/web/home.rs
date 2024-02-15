use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::song::SongHeader;

use super::AppState;

#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate {
    songs: Vec<SongHeader>,
}

pub async fn home(State(AppState { songs, .. }): State<AppState>) -> impl IntoResponse {
    return Html(
        HelloTemplate {
            songs: songs.all_songs(),
        }
        .render()
        .unwrap(),
    );
}

use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{song::Song, web::not_found};

use super::AppState;

#[derive(Deserialize)]
pub struct AddSong {
    author: String,
    title: String,
    contents: String,
}

pub async fn add_song(
    State(AppState { songs, .. }): State<AppState>,
    Form(payload): Form<AddSong>,
) -> Redirect {
    let song = Song::new(
        Uuid::new_v4(),
        payload.author,
        payload.title,
        payload.contents,
    );

    let id = song.id().clone();
    songs.add_song(song);

    return Redirect::to(&format!("/songs/{}", id));
}

#[derive(Template)]
#[template(path = "song.html")]
struct SongTemplate {
    song: Song,
}

pub async fn song(
    Path(id): Path<String>,
    State(AppState { songs, .. }): State<AppState>,
) -> impl IntoResponse {
    return Html(
        Uuid::parse_str(&id)
            .ok()
            .and_then(|song_id| songs.get_song(&song_id))
            .map(|song| SongTemplate { song: song.clone() }.render().unwrap())
            .unwrap_or_else(|| {
                println!("Could not find song for '{}'", &id);
                return not_found::not_found_html();
            }),
    );
}

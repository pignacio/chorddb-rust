use std::collections::{HashMap, HashSet};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chord::{finder::GUITAR_STANDARD, Chord},
    parser::{parse_tablature, Comp, Line, LineBit},
    song::{Song, SongHeader},
};

use super::AppState;

#[derive(Debug, PartialEq, Eq, Serialize)]
struct ChordModel {
    chord: String,
    fingering: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct LineBitModel {
    #[serde(rename = "type")]
    bit_type: String,
    position: usize,
    text: String,
    chord: Option<String>,
}

#[derive(Deserialize)]
pub struct AddSong {
    author: String,
    title: String,
    contents: String,
}

pub async fn add_song(State(AppState { songs, .. }): State<AppState>, payload: AddSong) -> Uuid {
    let id = Uuid::new_v4();
    let song = Song::new(id, payload.author, payload.title, payload.contents);

    songs.add_song(song);

    id
}

#[derive(Serialize)]
pub struct AddSongResult {
    success: bool,
    id: Uuid,
}

pub async fn api_add_song(
    state: State<AppState>,
    Json(payload): Json<AddSong>,
) -> Json<AddSongResult> {
    let id = add_song(state, payload).await;
    Json(AddSongResult { success: true, id })
}

fn serialize_line(line: &Line) -> Vec<LineBitModel> {
    line.iter().map(serialize_bit).collect()
}

lazy_static! {
    static ref FINGERING_BY_CHORD: HashMap<String, String> = [
        ("A", "002220"),
        ("Bb", "x13331"),
        ("B", "x24442"),
        ("C", "032010"),
        ("Db", "x466644"),
        ("D", "xx0232"),
        ("Eb", "xx1343"),
        ("E", "022100"),
        ("F", "133211"),
        ("Gb", "244322"),
        ("G", "320003"),
        ("Ab", "466544"),
        ("A7", "002020"),
        ("Bb7", "x13131"),
        ("B7", "x21202"),
        ("C7", "x35353"),
        ("Db7", "x46464"),
        ("D7", "xx0212"),
        ("Eb7", "xx1323"),
        ("E7", "020130"),
        ("F7", "131211"),
        ("Gb7", "242322"),
        ("G7", "320001"),
        ("Ab7", "464544"),
        ("Am", "002210"),
        ("Bbm", "x13321"),
        ("Bm", "x24432"),
        ("Cm", "035543"),
        ("Dbm", "x466544"),
        ("Dm", "xx0231"),
        ("Ebm", "xx1342"),
        ("Em", "022000"),
        ("Fm", "133111"),
        ("Gbm", "244222"),
        ("Gm", "355333"),
        ("Abm", "466444"),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_owned(), b.to_owned()))
    .collect();
}

fn serialize_bit(bit: &LineBit) -> LineBitModel {
    match &bit.comp {
        Comp::Text(text) => LineBitModel {
            bit_type: "text".to_owned(),
            position: bit.position,
            text: text.clone(),
            chord: None,
        },
        Comp::Chord {
            chord,
            original_text,
        } => LineBitModel {
            bit_type: "chord".to_owned(),
            position: bit.position,
            text: original_text.clone(),
            chord: Some(chord.text()),
        },
    }
}

pub async fn songs(State(AppState { songs, .. }): State<AppState>) -> impl IntoResponse {
    Json(songs.all_songs())
}

#[derive(Serialize)]
struct SongModel {
    header: SongHeader,
    contents: String,
    tablature: Vec<Vec<LineBitModel>>,
    fingerings: HashMap<String, String>,
}

fn extract_chords(tablature: Vec<Vec<LineBit>>) -> HashSet<Chord> {
    tablature
        .iter()
        .flatten()
        .filter_map(|b| match b.comp {
            Comp::Chord { chord, .. } => Some(chord),
            _ => None,
        })
        .collect()
}

pub async fn api_song(
    Path(id): Path<String>,
    State(AppState { songs, chords, .. }): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    return Uuid::parse_str(&id)
        .ok()
        .and_then(|song_id| songs.get_song(&song_id))
        .map(|song| {
            let tab = parse_tablature(song.contents());
            let serialized_tab = tab.iter().map(serialize_line).collect();

            let fingerings: HashMap<String, String> = extract_chords(tab)
                .iter()
                .filter_map(|c| {
                    chords
                        .get_fingerings(&GUITAR_STANDARD, c)
                        .first()
                        .map(|f| (c.text(), f.to_str()))
                })
                .collect();

            SongModel {
                header: song.header().clone(),
                contents: song.contents().into(),
                tablature: serialized_tab,
                fingerings,
            }
        })
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND);
}

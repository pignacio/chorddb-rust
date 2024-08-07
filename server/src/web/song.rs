use std::collections::{HashMap, HashSet};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chord::{finder::GUITAR_STANDARD, Chord},
    error::{ChordDbError, ChordDbResult},
    parser::{parse_tablature, Comp, Line, LineBit},
    song::{SeaOrmSongs, Song, SongHeader},
    user::User,
};

use super::{api::SimpleApiResult, AppState};

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

pub async fn add_song(
    AppState { songs, .. }: &AppState,
    user: &User,
    payload: AddSong,
) -> ChordDbResult<Uuid> {
    let id = Uuid::new_v4();
    let song = Song::new(id, payload.author, payload.title, payload.contents, user);

    songs.upsert_song(song).await?;

    Ok(id)
}

#[derive(Serialize)]
pub struct AddSongResult {
    success: bool,
    id: Uuid,
}

pub async fn api_add_song(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<AddSong>,
) -> ChordDbResult<Json<AddSongResult>> {
    add_song(&state, &user, payload)
        .await
        .map(|id| AddSongResult { success: true, id })
        .map(Json)
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

pub async fn songs(
    State(AppState { songs, .. }): State<AppState>,
    Extension(user): Extension<User>,
) -> ChordDbResult<Json<Vec<SongHeader>>> {
    Ok(Json(
        songs
            .all_songs()
            .await?
            .into_iter()
            .filter(|s| user_can_access(&user, s))
            .collect(),
    ))
}

#[derive(Serialize)]
struct SongModel {
    header: SongHeader,
    contents: String,
    tablature: Vec<Vec<LineBitModel>>,
    fingerings: HashMap<String, String>,
    original: String,
    instrument: String,
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

#[derive(Deserialize)]
pub struct SongQueryString {
    instrument: Option<String>,
}

pub async fn api_song(
    Path(id): Path<String>,
    Query(query_string): Query<SongQueryString>,
    Extension(user): Extension<User>,
    State(AppState {
        songs,
        chords,
        instruments,
        ..
    }): State<AppState>,
) -> ChordDbResult<impl IntoResponse> {
    let song = load_song(&id, &user, &songs).await?;

    let instrument = if let Some(instrument_id) = query_string.instrument {
        instruments
            .get_instrument(&instrument_id)
            .await
            .unwrap_or(GUITAR_STANDARD.clone())
    } else {
        GUITAR_STANDARD.clone()
    };
    let tab = parse_tablature(song.contents());
    let serialized_tab = tab.iter().map(serialize_line).collect();

    let fingerings: HashMap<String, String> = extract_chords(tab)
        .iter()
        .filter_map(|c| {
            chords
                .get_fingerings(&instrument, c)
                .first()
                .map(|f| (c.text(), f.to_str()))
        })
        .collect();

    let model = SongModel {
        header: song.header().clone(),
        contents: song.contents().into(),
        tablature: serialized_tab,
        fingerings,
        original: song.contents().to_string(),
        instrument: instrument.id().to_string(),
    };

    Ok(Json(model))
}

#[derive(Deserialize)]
pub struct SongDetails {
    author: Option<String>,
    title: Option<String>,
    contents: Option<String>,
}

impl SongDetails {
    fn is_empty(&self) -> bool {
        self.author.is_none() && self.title.is_none() && self.contents.is_none()
    }
}

async fn load_song(id: &str, user: &User, songs: &SeaOrmSongs) -> ChordDbResult<Song> {
    let Some(uuid) = Uuid::parse_str(id).ok() else {
        return Err(ChordDbError::HttpNotFound);
    };
    let Some(song) = songs.get_song(&uuid).await? else {
        return Err(ChordDbError::HttpNotFound);
    };
    if !user_can_access(user, &song.header) {
        return Err(ChordDbError::Forbidden);
    }

    Ok(song)
}

pub async fn delete_song(
    State(AppState { songs, .. }): State<AppState>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> ChordDbResult<Json<SimpleApiResult>> {
    let song = load_song(&id, &user, &songs).await?;

    songs.delete_song(song.id()).await?;

    Ok(Json(SimpleApiResult::simple_success("Delete successful")))
}

pub async fn patch_song(
    State(AppState { songs, .. }): State<AppState>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    Json(payload): Json<SongDetails>,
) -> ChordDbResult<Json<SimpleApiResult>> {
    let mut song = load_song(&id, &user, &songs).await?;
    if payload.is_empty() {
        return Err(ChordDbError::BadRequest(
            "All song fields where empty".to_string(),
        ));
    }

    if let Some(author) = payload.author {
        song.header.author = author
    }
    if let Some(title) = payload.title {
        song.header.title = title
    }
    if let Some(contents) = payload.contents {
        song.contents = contents
    }

    songs.upsert_song(song).await?;

    Ok(Json(SimpleApiResult::simple_success("Patch successful")))
}

fn user_can_access(user: &User, song: &SongHeader) -> bool {
    user.id == song.owner_id || user.is_admin
}

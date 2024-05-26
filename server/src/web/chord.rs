use axum::{
    extract::{Path, State},
    Json,
};

use crate::chord::{
    finder::{Fingering, StringInstrument, MIMI},
    Chord,
};

use super::AppState;

pub async fn chords(
    Path((instrument, chord)): Path<(String, String)>,
    State(AppState {
        chords,
        instruments,
        ..
    }): State<AppState>,
) -> Json<Vec<String>> {
    let Some(chord) = Chord::parse(chord) else {
        return Json(vec![]);
    };
    let Some(instrument) = instruments.get_instrument(&instrument).await else {
        return Json(vec![]);
    };
    let response = chords
        .get_fingerings(&instrument, &chord)
        .iter()
        .map(Fingering::to_str)
        .collect();
    Json::<Vec<_>>(response)
}

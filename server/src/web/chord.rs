use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::chord::{finder::Fingering, Chord};

use super::AppState;

#[derive(Serialize)]
pub struct FingeringModel {
    frets: Vec<String>,
    joined: String,
}

impl Into<FingeringModel> for &Fingering {
    fn into(self) -> FingeringModel {
        FingeringModel {
            frets: self.frets(),
            joined: self.to_str(),
        }
    }
}

pub async fn chords(
    Path((instrument, chord)): Path<(String, String)>,
    State(AppState {
        chords,
        instruments,
        ..
    }): State<AppState>,
) -> Json<Vec<FingeringModel>> {
    let Some(chord) = Chord::parse(chord) else {
        return Json(vec![]);
    };
    let Some(instrument) = instruments.get_instrument(&instrument).await else {
        return Json(vec![]);
    };
    let response = chords
        .get_fingerings(&instrument, &chord)
        .iter()
        .map(|f| f.into())
        .collect();
    Json::<Vec<_>>(response)
}

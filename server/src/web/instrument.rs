use axum::Json;
use serde::Serialize;

use crate::chord::finder::{StringInstrument, GUITAR_STANDARD, MIMI};

#[derive(Serialize)]
pub struct InstrumentModel {
    id: String,
    name: String,
    description: String,
}

pub async fn get_instruments() -> Json<Vec<InstrumentModel>> {
    let instruments: Vec<&'static StringInstrument> = vec![&GUITAR_STANDARD, &MIMI];
    Json(
        instruments
            .iter()
            .map(|instrument| InstrumentModel {
                id: instrument.id().to_string(),
                name: instrument.name().to_string(),
                description: instrument.description().to_string(),
            })
            .collect(),
    )
}

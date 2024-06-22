use axum::async_trait;
use dashmap::DashMap;

use crate::chord::finder::{StringInstrument, GUITAR_STANDARD, MIMI};

#[async_trait]
pub trait Instruments {
    async fn get_instrument(&self, id: &str) -> Option<StringInstrument>;
}

pub struct MemoryInstruments {
    instruments: DashMap<String, StringInstrument>,
}

impl MemoryInstruments {
    pub fn new() -> Self {
        let instruments = DashMap::new();
        instruments.insert("guitar".to_string(), GUITAR_STANDARD.clone());
        instruments.insert("mimi".to_string(), MIMI.clone());

        Self { instruments }
    }
}

impl Default for MemoryInstruments {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Instruments for MemoryInstruments {
    async fn get_instrument(&self, id: &str) -> Option<StringInstrument> {
        self.instruments.get(id).map(|r| r.value().clone())
    }
}

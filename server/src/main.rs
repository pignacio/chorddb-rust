use std::sync::Arc;

use chorddb::chord::finder::GUITAR_STANDARD;
use chorddb::song::{FileSongs, PrecomputedChords};
use chorddb::web::{run_server, AppState};
use chorddb::Opt;
use clap::Parser;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{},hyper=info,mio=info", opt.log_level()),
        )
    }
    // Enable console logging
    tracing_subscriber::fmt::init();

    let songs = FileSongs::new("songs.json");
    let state = AppState {
        songs: Arc::new(songs),
        chords: Arc::new(PrecomputedChords::new(&GUITAR_STANDARD)),
    };

    run_server(opt, state).await;
}

use std::sync::Arc;

use chorddb::chord::finder::{Fingering, GUITAR_STANDARD};
use chorddb::song::{CachedChords, FingeringCalculator, PrecomputedChords, SeaOrmSongs};
use chorddb::web::{run_server, AppState};
use chorddb::Opt;
use clap::Parser;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
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

    let url = std::env::var("DATABASE_URL").expect("Must set DATABASE_URL");
    let songs = SeaOrmSongs::new(
        Database::connect(url)
            .await
            .expect("Could not connect to the database"),
    );
    let state = AppState {
        songs: Arc::new(songs),
        chords: Arc::new(CachedChords::new(FingeringCalculator {})),
    };

    run_server(opt, state).await;
}

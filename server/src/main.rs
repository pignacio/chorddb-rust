use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use chorddb::instrument::MemoryInstruments;
use chorddb::session::SeaOrmSessions;
use chorddb::song::{CachedChords, FingeringCalculator, SeaOrmSongs};
use chorddb::user::SeaOrmUsers;
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

    create_sqlite_db_if_missing(&url);

    let db = Database::connect(&url)
        .await
        .unwrap_or_else(|err| panic!("Could not connect to the database @{}: {}", url, err));
    let state = AppState {
        songs: Arc::new(SeaOrmSongs::new(db.clone())),
        users: Arc::new(SeaOrmUsers::new(db.clone())),
        sessions: Arc::new(SeaOrmSessions::new(db.clone())),
        chords: Arc::new(CachedChords::new(FingeringCalculator {})),
        instruments: Arc::new(MemoryInstruments::new()),
    };

    run_server(opt, state).await;
}

const SQLITE_PREFIX: &str = "sqlite://";

fn create_sqlite_db_if_missing(url: &str) {
    if !url.starts_with(SQLITE_PREFIX) {
        return;
    }
    let path = Path::new(&url[SQLITE_PREFIX.len()..]);
    if !path.exists() {
        log::info!("Creating empty database @{}", url);
        File::create(path)
            .unwrap_or_else(|err| panic!("Could not create database file @ {}: {}", url, err));
    }
}

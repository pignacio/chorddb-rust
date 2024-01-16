use std::sync::Arc;

use chorddb::Opt;
use chorddb::song::FileSongs;
use chorddb::web::{AppState, run_server};
use clap::Parser;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level()))
    }
    // Enable console logging
    tracing_subscriber::fmt::init();
    
    let songs = FileSongs::new("songs.json");
    let state = AppState {
        songs: Arc::new(songs),
    };

    run_server(opt, state).await;
}

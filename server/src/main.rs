use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{net::{IpAddr, Ipv6Addr, SocketAddr}, fs::File};
use std::str::FromStr;
use std::sync::Arc;
use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use uuid::Uuid;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[derive(Clone)]
struct SongHeader {
    pub id: Uuid,
    pub author: String,
    pub title: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Song {
    pub id: Uuid,
    pub author: String,
    pub title: String,
    pub contents: String,
}
trait SongRepository {
    fn all_songs(&self) -> Vec<SongHeader>;
    fn add_song(&self, song: Song);
    fn get_song(&self, id: &Uuid) -> Option<Song>;
}

struct MemorySongs {
    songs: RwLock<HashMap<Uuid, Song>>,
}

impl MemorySongs {
    pub fn new() -> Self {
        return MemorySongs {
            songs: HashMap::new().into(),
        };
    }

    fn read_songs(&self) -> RwLockReadGuard<'_, HashMap<Uuid, Song>> {
        return self.songs.read().unwrap();
    }

    fn write_songs(&self) -> RwLockWriteGuard<'_, HashMap<Uuid, Song>> {
        return self.songs.write().unwrap();
    }
}

impl SongRepository for MemorySongs {
    fn all_songs(&self) -> Vec<SongHeader> {
        let songs = self.read_songs();
        songs
            .values()
            .map(|song| SongHeader {
                id: song.id.clone(),
                author: song.author.clone(),
                title: song.title.clone(),
            })
            .collect()
    }

    fn get_song(&self, id: &Uuid) -> Option<Song> {
        let songs = self.read_songs();
        return songs.get(id).cloned();
    }

    fn add_song(&self, song: Song) {
        let mut songs = self.write_songs();
        songs.insert(song.id.clone(), song);
    }
}

struct FileSongs {
    path: String,
    cache: MemorySongs,
}

fn load_cache(path: &str) -> MemorySongs {
    let data : Vec<Song> = std::fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or(Vec::new());

    let songs = MemorySongs::new();

    for song in data {
        songs.add_song(song)
    }
    
    return songs;
}

impl FileSongs {
    pub fn new(path: &str) -> Self {
        FileSongs{
            path: path.to_owned(),
            cache: load_cache(path),
        }
    }
    
    fn save_cache(&self) {
        let songs: Vec<Song> = self.cache.all_songs().iter()
            .filter_map(|header| self.cache.get_song(&header.id))
            .collect();
        std::fs::write(&self.path, serde_json::to_string(&songs).unwrap()).unwrap();
    }
}

impl SongRepository for FileSongs {
    fn all_songs(&self) -> Vec<SongHeader> {
        self.cache.all_songs()
    }

    fn add_song(&self, song: Song) {
        self.cache.add_song(song);
        self.save_cache();
    }

    fn get_song(&self, id: &Uuid) -> Option<Song> {
        self.cache.get_song(id)
    }
}

#[derive(Clone)]
struct AppState {
    pub songs: Arc<dyn SongRepository + Send + Sync>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // Enable console logging
    tracing_subscriber::fmt::init();

    // let songs = MemorySongs::new();
    // songs.add_song(Song {
    //     id: Uuid::new_v4(),
    //     author: "Some author".to_owned(),
    //     title: "Some title".to_owned(),
    //     contents: "1234567890".to_owned(),
    // });

    let songs = FileSongs::new("songs.json");
    
    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/", get(home))
        .route("/songs", post(add_song))
        .route("/songs/:id", get(song))
        .nest_service("/dist", ServeDir::new("dist"))
        .fallback(not_found)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(AppState {
            songs: Arc::new(songs),
        });

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate<'a> {
    name: &'a str,
    songs: Vec<SongHeader>,
}

async fn home(State(AppState { songs, .. }): State<AppState>) -> impl IntoResponse {
    return Html(format!(
        "{}",
        HelloTemplate {
            name: "Nacho",
            songs: songs.all_songs()
        }
        .render()
        .unwrap()
    ));
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {}

async fn not_found() -> Html<String> {
    return Html(format!("{}", NotFoundTemplate {}.render().unwrap()));
}

#[derive(Deserialize)]
struct AddSong {
    author: String,
    title: String,
    contents: String,
}

async fn add_song(
    State(AppState { songs, .. }): State<AppState>,
    Form(payload): Form<AddSong>,
) -> Redirect {
    let song = Song {
        id: Uuid::new_v4(),
        author: payload.author,
        title: payload.title,
        contents: payload.contents,
    };

    let id = song.id.clone();
    songs.add_song(song);

    return Redirect::to(&format!("/songs/{}", id));
}

#[derive(Template)]
#[template(path = "song.html")]
struct SongTemplate {
    song: Song,
}

async fn song(
    Path(id): Path<String>, State(AppState { songs, .. }): State<AppState>
) -> impl IntoResponse {
    return Uuid::parse_str(&id).ok()
        .and_then(|song_id| songs.get_song(&song_id))
        .map(|song| Html(format!("{}", SongTemplate { song: song.clone() }.render().unwrap())))
        .unwrap_or_else(|| {
            println!("Could not find song for '{}'", &id);
            return Html(format!("{}", NotFoundTemplate {}.render().unwrap()))
        });
}

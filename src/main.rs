use std::sync::{Arc, Mutex, MutexGuard};

use leptos::*;
use leptos_router::*;

mod views;

fn main() {
    // println!("Main!");
    // if std::env::var("RUST_LOG").is_err() {
    //     std::env::set_var("RUST_LOG", "info");
    // }
    // log::info!("Starting!!");
    // env_logger::builder()
    //     .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
    //     .init();
    //
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! { <MainRouter/> }
}

pub(crate) trait Songs: Send {
    fn get_songs(&self) -> Vec<String>;
}

#[derive(Debug, Clone)]
struct FileSongs {
    path: String,
    songs: Vec<String>,
}

impl FileSongs {
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        let path = path.as_ref().to_string();

        FileSongs {
            path,
            songs: vec!["Some Title".to_string(), "Another Title".to_string()],
        }
    }
}

impl Songs for FileSongs {
    fn get_songs(&self) -> Vec<String> {
        self.songs.clone()
    }
}

pub(crate) struct AppState {
    songs: Mutex<Box<dyn Songs + 'static>>,
}

impl AppState {
    pub(crate) fn new<S: Songs + 'static>(songs: S) -> Self {
        Self {
            songs: Mutex::new(Box::new(songs)),
        }
    }

    pub(crate) fn songs(&self) -> MutexGuard<Box<dyn Songs>> {
        AppState::lock_clearing(&self.songs, "Songs")
    }

    fn lock_clearing<'a, T>(mutex: &'a Mutex<T>, name: &'static str) -> MutexGuard<'a, T> {
        mutex.lock().unwrap_or_else(|err| {
            log::info!("Clearing poison for {}", name);
            mutex.clear_poison();
            err.into_inner()
        })
    }
}

#[component]
fn MainRouter() -> impl IntoView {
    let songs = FileSongs::new("songs.json");
    let state = AppState::new(songs);
    let arc: Arc<AppState> = Arc::new(state);

    log::info!("MainRouter!");

    provide_context(arc);

    view! {
        <Router>
            <div class="navbar bg-neutral text-neutral-content">
                <div class="navbar-start">
                    <a href="/" class="btn btn-ghost text-xl">
                        ChordDB
                    </a>
                </div>
            </div>

            <div class="container mx-auto p-4">
                <Routes>
                    <Route path="/" view=views::Home />
                    <Route path="/songs" view=|| view! { Song }/>
                    <Route path="" view=|| view! { Default }/>
                </Routes>
            </div>
        </Router>
    }
}

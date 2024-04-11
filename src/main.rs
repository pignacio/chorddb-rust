use std::sync::Arc;

use leptos::*;
use leptos_router::*;

mod service;
mod state;
mod views;

use service::{FileSongs, Songs};
use state::ServerState;

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

#[component]
fn MainRouter() -> impl IntoView {
    let songs = FileSongs::new("songs.json");
    let state = ServerState::new(songs);
    let arc: Arc<ServerState> = Arc::new(state);

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

use std::sync::Arc;

use leptos::{component, expect_context, view, IntoView};

use crate::{AppState, Songs};

#[component]
pub fn Home() -> impl IntoView {
    leptos::logging::log!("Home!");
    let bad: Arc<dyn Songs + 'static> = expect_context();
    let state: Arc<AppState> = expect_context();
    let songs = state.songs();
    view! {
        <h1>Home</h1>

        {songs.get_songs()}
    }
}

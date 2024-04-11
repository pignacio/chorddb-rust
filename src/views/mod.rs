use std::sync::Arc;

use leptos::{
    component, create_resource, expect_context, server, view, IntoView, ServerFnError, SignalGet,
};

use crate::{state::get_server_state_or_fail, ServerState, Songs};

#[server(GetSongHeaders, "/api", "GetJson")]
async fn get_song_headers() -> Result<Vec<String>, ServerFnError> {
    Ok(get_server_state_or_fail().songs().get_songs());
}

#[component]
pub fn Home() -> impl IntoView {
    let songs = create_resource(|| (), |_| async move { get_song_headers().await });
    view! {
        <h1>Home</h1>

        {move || match songs.get() {
        None => view! { <p>"Loading..."</p> }.into_view(),
        Some(titles) => view! { {titles} }.into_view()
    }}
    }
}

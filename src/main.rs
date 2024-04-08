use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! { <MainRouter/> }
}

#[component]
fn MainRouter() -> impl IntoView {
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
                    <Route path="/" view=|| view! { Home }/>
                    <Route path="/songs" view=|| view! { Song }/>
                    <Route path="" view=|| view! { Default }/>
                </Routes>
            </div>
        </Router>
    }
}

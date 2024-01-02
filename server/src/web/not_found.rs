use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {}

pub async fn not_found() -> Html<String> {
    return Html(not_found_html());
}

pub fn not_found_html() -> String {
    return NotFoundTemplate {}.render().unwrap();
}

mod hello_world;
mod get_body_text;
use axum::{routing::{get, post}, Router};
use get_body_text::body_text;
use hello_world::hello_world;

pub fn create_routes() -> Router {
    Router::new()
    .route("/", get(hello_world))
    .route("/body-text", post(body_text))
}


mod hello_world;
mod get_body_text;
mod body_json;
use axum::{routing::{get, post}, Router};
use body_json::{json_body, json_body2};
use get_body_text::body_text;
use hello_world::hello_world;

pub fn create_routes() -> Router {
    Router::new()
    .route("/", get(hello_world))
    .route("/body-text", post(body_text))
    .route("/body-json", post(json_body))
    .route("/body-json2", post(json_body2))
}


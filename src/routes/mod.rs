mod hello_world;
mod get_body_text;
mod body_json;
mod path_params;
mod query_params;
mod get_headers;

use axum::{routing::{get, post}, Router};
use body_json::{json_body, json_body2};
use get_body_text::body_text;
use get_headers::get_header;
use hello_world::hello_world;
use path_params::path_param;
use query_params::query_params;

pub fn create_routes() -> Router {
    Router::new()
    .route("/", get(hello_world))
    .route("/body-text", post(body_text))
    .route("/body-json", post(json_body))
    .route("/body-json2", post(json_body2))
    .route("/path/:id", get(path_param))
    .route("/query", get(query_params))
    .route("/headers", get(get_header))
}


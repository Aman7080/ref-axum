mod hello_world;
mod get_body_text;
mod body_json;
mod path_params;
mod query_params;
mod get_headers;

use axum::{http::Method, routing::{get, post}, Router};
use body_json::{json_body, json_body2};
use get_body_text::body_text;
use get_headers::get_header;
use hello_world::hello_world;
use path_params::path_param;
use query_params::query_params;
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {
    
    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any);

    Router::new()
    .route("/", get(hello_world))
    .route("/body-text", post(body_text))
    .route("/body-json", post(json_body))
    .route("/body-json2", post(json_body2))
    .route("/path/:id", get(path_param))
    .route("/query", get(query_params))
    .route("/headers", get(get_header))
    .layer(cors)
    .layer(TraceLayer::new_for_http())
    
}


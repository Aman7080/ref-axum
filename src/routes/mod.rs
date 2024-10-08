mod handlers;
use axum::{routing::get, Router};
use handlers::hello_world;

pub fn create_routes() -> Router {
    Router::new().route("/", get(hello_world))
}


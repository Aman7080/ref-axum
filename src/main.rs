use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(hello_world));
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn hello_world() -> String{
     "hello world".to_owned()
}

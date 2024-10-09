mod routes;
use axum::Router;
use routes::create_routes;

pub async fn run(){
    let app: Router = create_routes();

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}




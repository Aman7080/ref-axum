use axum_rust::run;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        // .with(
        //     tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        //         format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        //     }),
        // )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    run().await
}           
use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

use tracing_subscriber::EnvFilter;

use tracing::info;

async fn home() -> Html<&'static str> {
    Html("Hello world")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .compact()
        .init();

    let app = Router::new()
        .route("/", get(home));

    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("invalid socket address");
    info!("listening on http://{}", addr.to_string());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

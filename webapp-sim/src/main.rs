use std::net::SocketAddr;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{routing::get, Router};

use tracing_subscriber::EnvFilter;

use tracing::info;


#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn home() -> Response {
    let hello = HelloTemplate { name: "world" };
    hello.into_response()
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

use std::net::SocketAddr;
use std::sync::Arc;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{extract::State, routing::get, Router};
use axum_macros::debug_handler;
use tokio::sync::RwLock;

use tracing_subscriber::EnvFilter;

use tracing::info;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[derive(Clone)]
struct AppState {
    data: Vec<u8>
}

#[debug_handler]
async fn home(State(state): State<AppState>) -> Response {
    let data = state.data[0];
    let hello = HelloTemplate { name: &data.to_string() };
    hello.into_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .compact()
        .init();

    let state = AppState { data: vec![1,2,3,4] };

    let app = Router::new()
        .route("/", get(home))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("invalid socket address");
    info!("listening on http://{}", addr.to_string());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

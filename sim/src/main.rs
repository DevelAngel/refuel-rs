use std::net::SocketAddr;
use std::sync::Arc;

use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::{extract::State, routing::get, Router};
use axum_macros::debug_handler;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tokio::try_join;

use tracing_subscriber::EnvFilter;

use tracing::{info, error};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
    data: &'a [u8],
}

#[derive(Clone)]
struct AppState {
    data: Vec<u8>
}

#[debug_handler]
async fn home(State(state): State<Arc<RwLock<AppState>>>) -> Response {
    let state = state.read().await;
    let data = &state.data;
    let hello = HelloTemplate { name: &data.len().to_string(), data: &data };
    hello.into_response()
}

async fn change_state(state: Arc<RwLock<AppState>>) -> Result<(), hyper::Error> {
    loop {
        sleep(Duration::from_millis(5000)).await;
        let mut state = state.write().await;
        state.data.push(9);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .compact()
        .init();

    let state = AppState { data: vec![1,2,3,4] };
    let state = Arc::new(RwLock::new(state));

    let app = Router::new()
        .route("/", get(home))
        .with_state(Arc::clone(&state));

    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("invalid socket address");
    info!("listening on http://{}", addr.to_string());
    let service = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    let change_state = change_state(Arc::clone(&state));

    match try_join!(service, change_state) {
        Ok(_) => {info!("good bye");}
        Err(_) => {error!("bah, good bye");}
    }

    Ok(())
}

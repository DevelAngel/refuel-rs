use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    response::IntoResponse,
    http::{Request, Response, StatusCode, Uri},
};
use axum::response::Response as AxumResponse;
use axum_macros::debug_handler;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use leptos::LeptosOptions;

#[debug_handler]
pub async fn file_and_error_handler(uri: Uri, State(options): State<LeptosOptions>, _req: Request<Body>) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    res.into_response()
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "web/dist/"]
struct FrontendAssets;

pub async fn serve_frontend(uri: axum::http::Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    // Try to serve the exact file
    if let Some(file) = FrontendAssets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, mime.as_ref())],
            file.data.into_owned(),
        )
            .into_response()
    } else {
        // SPA fallback: serve index.html for any non-file route
        match FrontendAssets::get("index.html") {
            Some(file) => Html(String::from_utf8_lossy(&file.data).to_string()).into_response(),
            None => (StatusCode::NOT_FOUND, "Frontend not built").into_response(),
        }
    }
}

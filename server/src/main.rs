use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{Any, CorsLayer};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let layer = CorsLayer::new().allow_headers(Any);
    let router = Router::new().nest_service(
        "/",
        ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
    ).layer(layer);
    Ok(router.into())
}

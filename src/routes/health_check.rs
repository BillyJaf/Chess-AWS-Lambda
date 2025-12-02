use axum::{
    Router, http::{HeaderValue, Method, header::CONTENT_TYPE}, routing::get
};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::handlers::health_check;

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            HeaderValue::from_static("https://frontend-chess-bot.netlify.app"), // prod
            HeaderValue::from_static("http://localhost:5173"), // dev
        ]))
        .allow_methods([Method::GET])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/", get(health_check::health_check))
        .layer(cors)
}
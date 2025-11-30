use axum::{
    Router, http::{Method, header::CONTENT_TYPE}, routing::get
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::health_check;

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/", get(health_check::health_check))
        .layer(cors)
}
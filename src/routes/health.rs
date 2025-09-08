use axum::{
    routing::get, 
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::health;

pub fn routes() -> Router {
    // THIS ALLOWS ALL CONNECTIONS, ONLY USE THIS FOR DEV
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new().route("/", get(health::health_check)).layer(cors)
}
use axum::{
    routing::post,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::legal_moves;

pub fn routes() -> Router {
    // THIS ALLOWS ALL CONNECTIONS, ONLY USE THIS FOR DEV
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new().route("/", post(legal_moves::legal_moves)).layer(cors) 
}
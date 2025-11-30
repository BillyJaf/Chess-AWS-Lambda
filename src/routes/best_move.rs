use axum::{
    Router, http::{Method, header::CONTENT_TYPE}, routing::post
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::best_move;

pub fn routes() -> Router {
    // THIS ALLOWS ALL CONNECTIONS, ONLY USE THIS FOR DEV
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/", post(best_move::best_move))
        .layer(cors) 
}
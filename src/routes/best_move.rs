use axum::{
    Router, http::{HeaderValue, Method, header::CONTENT_TYPE}, routing::post
};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::handlers::best_move;

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            HeaderValue::from_static("https://frontend-chess-bot.netlify.app"), // prod
            HeaderValue::from_static("http://localhost:5173"), // dev
        ]))
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/", post(best_move::best_move))
        .layer(cors) 
}
use axum::Router;

pub mod health_check;
pub mod legal_moves;
pub mod best_move;

pub fn app_routes() -> Router {
    Router::new()
        .nest("/health_check", health_check::routes())
        .nest("/legal_moves", legal_moves::routes())
        .nest("/best_move", best_move::routes())
}
use axum::{ 
    http::StatusCode, 
    response::IntoResponse, 
    Json 
};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

pub async fn health_check() -> impl IntoResponse {
    let body = HealthResponse { status: String::from("ok") };
    (StatusCode::OK, Json(body))
}
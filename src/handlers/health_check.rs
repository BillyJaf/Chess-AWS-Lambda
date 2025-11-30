use axum::{ 
    http::StatusCode, 
    response::IntoResponse, 
    Json 
};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn health_check() -> impl IntoResponse {
    println!("Received a Health Check!");
    let body = HealthResponse { status: "ok" };
    (StatusCode::OK, Json(body))
}
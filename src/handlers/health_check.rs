use crate::types::HealthResponse;

pub async fn health_check() -> HealthResponse {
    HealthResponse { status: String::from("ok") }
}
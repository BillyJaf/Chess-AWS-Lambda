use axum::{ 
    response::{ Response, IntoResponse }, Json
 };
use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseError {
    pub error: String,
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
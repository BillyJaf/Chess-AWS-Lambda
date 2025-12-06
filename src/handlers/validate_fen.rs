use axum::{ 
    http::StatusCode, 
    response::IntoResponse, 
    Json 
};
use pleco::Board;
use serde::Serialize;

#[derive(Serialize)]
struct ValidateFenResponse {
    valid: bool,
    error: Option<String>,
}

pub async fn validate_fen(Json(fen_input): Json<String>) -> impl IntoResponse {
    match Board::from_fen(&fen_input) {
        Ok(_) => (StatusCode::OK, Json(ValidateFenResponse {valid: true, error: None})).into_response(),
        Err(e) => {
            let error = format!("{:?}",e);
            (StatusCode::OK, Json(ValidateFenResponse {valid: false, error: Some(error)})).into_response()
        },
    }
}
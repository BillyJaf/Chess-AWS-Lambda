use lambda_http::Body;
use pleco::Board;

use crate::{error::ResponseError, types::ValidateFenResponse};

pub async fn validate_fen(body: Body) -> Result<ValidateFenResponse, ResponseError> {
    let body_bytes = match body {
        Body::Text(s) => s.into_bytes(),
        Body::Binary(b) => b,
        Body::Empty => return Err(ResponseError { error: String::from("Empty body") })
    };

    let fen_input: String = serde_json::from_slice(&body_bytes)
        .map_err(|e| ResponseError { error: format!("Invalid JSON: {}", e)})?;

    match Board::from_fen(&fen_input) {
        Ok(_) => Ok(ValidateFenResponse {valid: true, error: None}),
        Err(e) => {
            let error = format!("{:?}",e);
            Ok(ValidateFenResponse {valid: false, error: Some(error)})
        }
    }
}
use pleco::Board;

use crate::{error::ResponseError, types::ValidateFenResponse};

pub async fn validate_fen(fen: String) -> Result<ValidateFenResponse, ResponseError> {
    match Board::from_fen(&fen) {
        Ok(_) => Ok(ValidateFenResponse {valid: true, error: None}),
        Err(e) => {
            let error = format!("{:?}",e);
            Ok(ValidateFenResponse {valid: false, error: Some(error)})
        }
    }
}
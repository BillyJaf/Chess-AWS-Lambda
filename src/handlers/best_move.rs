use axum::{ 
    http::StatusCode, response::{ Response, IntoResponse }, Json
 };
use pleco::Board;
use serde::Serialize;
use crate::{bot::move_generation::generate_best_move, error::ResponseError};
#[derive(Serialize)]
struct BestMoveResponse {
    checkmate: bool,
    stalemate: bool,
    uci_move: String,
    resulting_fen: String,
}

impl IntoResponse for BestMoveResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub async fn best_move(Json(fen_input): Json<String>) -> impl IntoResponse {
    println!("Calculating Best Move of: {}", fen_input);

    match Board::from_fen(&fen_input) {
        Ok(board) => {
            let best_move = generate_best_move(board, 5);

            let new_board = &best_move.resulting_board.unwrap();

            let best_move_response = BestMoveResponse {
                checkmate: new_board.checkmate(),
                stalemate: new_board.stalemate(),
                uci_move: best_move.uci_move.unwrap(),
                resulting_fen: new_board.fen(),
            };
            (StatusCode::OK, best_move_response).into_response()
        },
        Err(e) => {
            let error = ResponseError { error: format!("{:?}",e) };
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        },
    }
}
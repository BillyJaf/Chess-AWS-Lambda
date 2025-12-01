use axum::{ http::StatusCode, response::{ Response, IntoResponse }, Json };
use pleco::Board;
use serde::Serialize;
use crate::{error::ResponseError, handlers::{types::{GameOver, ResultingGameState}, utils::{game_over, get_resulting_game_states}}};

#[derive(Serialize)]
struct LegalMoves {
    game_over: Option<GameOver>,
    legal_moves: Vec<ResultingGameState>,
}

impl IntoResponse for LegalMoves {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

fn generate_legal_moves(mut board: Board) -> LegalMoves {
    let legal_moves= get_resulting_game_states(&mut board);

    LegalMoves { 
        game_over: game_over(&board),
        legal_moves
    }
}

pub async fn legal_moves(Json(fen_input): Json<String>) -> impl IntoResponse {
    match Board::from_fen(&fen_input) {
        Ok(board) => (StatusCode::OK, generate_legal_moves(board)).into_response(),
        Err(e) => {
            let error = ResponseError { error: format!("{:?}",e) };
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        },
    }
}
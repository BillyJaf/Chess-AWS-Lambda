use axum::{ http::StatusCode, response::{ Response, IntoResponse }, Json };
use pleco::Board;
use serde::Serialize;
use crate::{bot::engine::Engine, error::ResponseError, types::{GameOver, ResultingGameState}, utils::{game_over, get_resulting_game_states}};

#[derive(Serialize)]
struct BestMoveResponse {
    game_over: Option<GameOver>,
    uci_move: String,
    san_move: String,
    resulting_fen: String,
    resulting_legal_moves: Vec<ResultingGameState>
}

impl IntoResponse for BestMoveResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub async fn best_move(Json(fen_input): Json<String>) -> impl IntoResponse {

    let mut engine = match Board::from_fen(&fen_input) {
        Ok(board) => Engine::new(board),
        Err(e) => {
            let error = ResponseError { error: format!("{:?}",e) };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response()
        },
    };

    let option_best_move = tokio::task::spawn_blocking(move || {
        engine.best_move()
    }).await.unwrap();

    let best_move_response = match option_best_move {
        None => {
            let error = ResponseError { error: String::from("No Legal Moves.") };
            return (StatusCode::BAD_REQUEST, Json(error)).into_response()
        }

        Some(best_move) => { 
            let mut new_board = best_move.resulting_board;
            let resulting_legal_moves = get_resulting_game_states(&mut new_board);

            BestMoveResponse {
                game_over: game_over(&new_board),
                uci_move: best_move.uci_move,
                san_move: best_move.san_move,
                resulting_fen: new_board.fen(),
                resulting_legal_moves,
            }
        }
    };

    (StatusCode::OK, best_move_response).into_response()
}
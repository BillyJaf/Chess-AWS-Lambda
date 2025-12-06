use lambda_http::Body;
use pleco::Board;
use crate::{bot::engine::Engine, error::ResponseError, types::BestMoveResponse, utils::{game_over, get_resulting_game_states}};

pub async fn best_move(body: Body) -> Result<BestMoveResponse, ResponseError> {

    let body_bytes = match body {
        Body::Text(s) => s.into_bytes(),
        Body::Binary(b) => b,
        Body::Empty => return Err(ResponseError { error: String::from("Empty body") })
    };

    let fen_input: String = serde_json::from_slice(&body_bytes)
        .map_err(|e| ResponseError { error: format!("Invalid JSON: {}", e)})?;

    let board = Board::from_fen(&fen_input)
        .map_err(|e| ResponseError { error: format!("{:?}",e) })?;

    let mut engine = Engine::new(board);

    let option_best_move = tokio::task::spawn_blocking(move || {
        engine.best_move()
    }).await.unwrap();

    match option_best_move {
        None => Err(ResponseError { error: String::from("No Legal Moves.")}),

        Some(best_move) => { 
            let mut new_board = best_move.resulting_board;
            let resulting_legal_moves = get_resulting_game_states(&mut new_board);

            Ok(BestMoveResponse {
                game_over: game_over(&new_board),
                uci_move: best_move.uci_move,
                san_move: best_move.san_move,
                resulting_fen: new_board.fen(),
                resulting_legal_moves,
            })
        }
    }
}
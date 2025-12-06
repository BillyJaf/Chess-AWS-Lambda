use lambda_http::Body;
use pleco::Board;
use crate::{error::ResponseError, types::{LegalMoves}, utils::{game_over, get_resulting_game_states}};

fn generate_legal_moves(mut board: Board) -> LegalMoves {
    let legal_moves= get_resulting_game_states(&mut board);

    LegalMoves { 
        game_over: game_over(&board),
        legal_moves
    }
}

pub async fn legal_moves(body: Body) -> Result<LegalMoves, ResponseError> {
    let body_bytes = match body {
        Body::Text(s) => s.into_bytes(),
        Body::Binary(b) => b,
        Body::Empty => return Err(ResponseError { error: String::from("Empty body") })
    };

    let fen_input: String = serde_json::from_slice(&body_bytes)
        .map_err(|e| ResponseError { error: format!("Invalid JSON: {}", e)})?;

    let board = Board::from_fen(&fen_input)
        .map_err(|e| ResponseError { error: format!("Invalid FEN: {:?}", e)})?;

    Ok(generate_legal_moves(board))
}
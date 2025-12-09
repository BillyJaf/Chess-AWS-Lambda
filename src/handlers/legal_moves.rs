use pleco::Board;
use crate::{types::{LegalMoves, ResponseError}, utils::{game_over, get_resulting_game_states}};

fn generate_legal_moves(mut board: Board) -> LegalMoves {
    let legal_moves= get_resulting_game_states(&mut board);

    LegalMoves { 
        game_over: game_over(&board),
        legal_moves
    }
}

pub async fn legal_moves(fen: String) -> Result<LegalMoves, ResponseError> {
    let board = Board::from_fen(&fen)
        .map_err(|e| ResponseError { error: format!("{:?}", e)})?;

    Ok(generate_legal_moves(board))
}
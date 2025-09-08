use axum::{ 
    response::{ IntoResponse, Response }, 
    Json,
 };
use pleco::Board;
use serde::{ Serialize };

#[derive(Serialize)]
pub struct LegalMoves {
    moves: Vec<String>,
}

impl IntoResponse for LegalMoves {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub fn generate_legal_moves(board: Board) -> LegalMoves {
    let mut moves: Vec<String> = Vec::new();
    
    let legal_moves = board.generate_moves();
    for mv in legal_moves.iter() {
        moves.push(mv.stringify());
    }
    LegalMoves { moves: moves }
}
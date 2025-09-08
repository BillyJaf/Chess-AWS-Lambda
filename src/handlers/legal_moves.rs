use axum::{ 
    http::StatusCode, response::{ Response, IntoResponse }, Json
 };
use pleco::{Board, Player};
use serde::Serialize;
use crate::error::ResponseError;

pub type MoveFenTuple = (String, String);

#[derive(Serialize)]
struct LegalMoves {
    winner: Option<char>,
    stalemate: bool,
    moves: Vec<MoveFenTuple>,
}

impl IntoResponse for LegalMoves {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

fn generate_legal_moves(mut board: Board) -> LegalMoves {
    let mut moves: Vec<MoveFenTuple> = Vec::new();
    
    let legal_moves = board.generate_moves();

    for mov in legal_moves.iter() {
        board.apply_move(*mov);
        moves.push((mov.stringify(), board.fen()));
        board.undo_move();  
    }
    let stalemate = board.stalemate();
    let mut winner = None;

    if legal_moves.len() == 0 {
        winner = match board.turn() {
            Player::White => Some('b'),
            Player::Black => Some('w')
        };
    }

    LegalMoves { 
        winner,
        stalemate,
        moves
    }
}

pub async fn legal_moves(Json(fen_input): Json<String>) -> impl IntoResponse {
    println!("Calculating Legal Moves of: {}", fen_input);

    match Board::from_fen(&fen_input) {
        Ok(board) => (StatusCode::OK, generate_legal_moves(board)).into_response(),
        Err(e) => {
            let error = ResponseError { error: format!("{:?}",e) };
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        },
    }
}
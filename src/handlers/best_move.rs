use axum::{ 
    http::StatusCode, response::{ Response, IntoResponse }, Json
 };
use pleco::{Board, Player};
use serde::Serialize;
use crate::error::ResponseError;
use rand::thread_rng;
use rand::prelude::IteratorRandom;

#[derive(Serialize)]
struct BestMove {
    winner: Option<char>,
    stalemate: bool,
    uci_move: String,
    resulting_fen: String,
}

impl IntoResponse for BestMove {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

fn generate_best_move(mut board: Board) -> BestMove {
    
    let legal_moves = board.generate_moves();

    let mut uci_move = String::from("");

    let mut resulting_fen = String::from("");

    if legal_moves.len() > 0 {
        let mut rng = thread_rng();

        let bit_move = legal_moves.iter().choose(&mut rng).unwrap();
        
        uci_move = bit_move.stringify();

        board.apply_move(*bit_move);
        
        resulting_fen = board.fen();

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

    BestMove { 
        winner,
        stalemate,
        uci_move,
        resulting_fen
    }
}

pub async fn best_move(Json(fen_input): Json<String>) -> impl IntoResponse {
    println!("Calculating Best Move of: {}", fen_input);

    match Board::from_fen(&fen_input) {
        Ok(board) => (StatusCode::OK, generate_best_move(board)).into_response(),
        Err(e) => {
            let error = ResponseError { error: format!("{:?}",e) };
            (StatusCode::BAD_REQUEST, Json(error)).into_response()
        },
    }
}
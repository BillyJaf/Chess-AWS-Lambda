use axum::{ 
    http::StatusCode, response::{ Response, IntoResponse }, Json
 };
use pleco::Board;
use serde::Serialize;

#[derive(Serialize)]
struct LegalMoves {
    moves: Vec<String>,
}

impl IntoResponse for LegalMoves {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

fn generate_legal_moves(board: Board) -> LegalMoves {
    let mut moves: Vec<String> = Vec::new();
    
    let legal_moves = board.generate_moves();
    for mv in legal_moves.iter() {
        moves.push(mv.stringify());
    }
    LegalMoves { moves: moves }
}

pub async fn legal_moves(Json(fen_input): Json<String>) -> impl IntoResponse {
    println!("Calculating Legal Moves of: {}", fen_input);

    match Board::from_fen(&fen_input) {
        Ok(b) => (StatusCode::OK, generate_legal_moves(b)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("{:?}",e)).into_response(),
    }
}
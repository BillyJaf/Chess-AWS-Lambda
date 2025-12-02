use pleco::{Board, Player};

use crate::handlers::types::{GameOver, ResultingGameState};

pub fn game_over(board: &Board) -> Option<GameOver> {
    if board.stalemate() {
        return Some(GameOver::Stalemate)
    }
    if board.checkmate() {
        return match board.turn() {
            Player::White => Some(GameOver::Black),
            Player::Black => Some(GameOver::White)
        }
    }
    None
}

pub fn get_resulting_game_states(board: &mut Board) -> Vec<ResultingGameState> {
    let mut moves: Vec<ResultingGameState> = Vec::new();

    let legal_moves = board.generate_moves();

    for mv in legal_moves.iter() {
        board.apply_move(*mv);
        moves.push(ResultingGameState {
            uci_move: mv.stringify(),
            resulting_fen: board.fen(),
            game_over: game_over(board)
        });
        board.undo_move();  
    }

    moves
}
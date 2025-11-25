use pleco::{Board, MoveList, Player};

use crate::bot::heuristics::heuristic;

pub fn is_game_over(board: &Board) -> bool {
    board.checkmate() || board.stalemate()
}

// Orders a list of moves with best rated first.
// Return minus the evalution to sort in descending order (best move first).
pub fn order_moves_max(board: &mut Board, moves: &mut MoveList, bot_colour: Player) {
    moves.sort_by_key(|mv| {
        board.apply_move(*mv);
        let evaluation = heuristic(board, bot_colour);
        board.undo_move();
        -evaluation
    });
}

// Orders a list of moves with worst rated first.
// Return the evalution to sort in ascending order (worst move first).
pub fn order_moves_min(board: &mut Board, moves: &mut MoveList, bot_colour: Player) {
    moves.sort_by_key(|mv| {
        board.apply_move(*mv);
        let evaluation = heuristic(board, bot_colour);
        board.undo_move();
        evaluation
    });
}
use pleco::Board;

pub fn is_game_over(board: &Board) -> bool {
    board.checkmate() || board.stalemate()
}
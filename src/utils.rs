use pleco::{Board, Player};
use shakmaty::{Chess, fen::Fen, san::San, uci::UciMove, CastlingMode};

use crate::types::{GameOver, ResultingGameState};

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
        let uci_move: String = mv.stringify();
        let san_move: String = uci_to_san(&board.fen(), &uci_move);

        board.apply_move(*mv);
        moves.push(ResultingGameState {
            uci_move,
            san_move,
            resulting_fen: board.fen(),
            game_over: game_over(board)
        });
        board.undo_move();  
    }

    moves
}

pub fn uci_to_san(fen: &str, uci_move: &str) -> String {
    let fen_obj = Fen::from_ascii(fen.as_bytes()).unwrap();
    let position: Chess = fen_obj.into_position(CastlingMode::Standard).unwrap();
    let uci = UciMove::from_ascii(uci_move.as_bytes()).unwrap();
    let mv = uci.to_move(&position).unwrap();
    San::from_move(&position, mv).to_string()
}
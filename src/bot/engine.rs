use std::{i32};
use pleco::{BitMove, Board, Player};
use crate::{bot::heuristics::{heuristic, is_game_over}, types::{BestMove, MoveGenerationData}, utils::uci_to_san};

pub struct Engine {
    board: Board,
    search_depth: u8,
}

impl Engine {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            search_depth: 4,
        }
    }

    pub fn best_move(&mut self) -> Option<BestMove> {
        if is_game_over(&self.board) {
            return None
        }

        let mut current_board = self.board.clone();
        let bot_colour = self.board.turn();
        let node_height = self.search_depth;
        let root_height = self.search_depth;
        let alpha = i32::MIN;
        let beta = i32::MAX;

        let move_gen = self.search(&mut current_board, bot_colour, BitMove::null(), node_height, root_height, alpha, beta);

        let uci_move: String = move_gen.bit_move.stringify();
        let san_move: String = uci_to_san(&current_board.fen(), &uci_move);

        current_board.apply_move(move_gen.bit_move);

        Some(BestMove { 
            uci_move,
            san_move,
            resulting_board: current_board,
        })
    }

    fn search(&self, board: &mut Board, bot_colour: Player, move_made: BitMove, node_height: u8, root_height: u8, mut alpha: i32, mut beta: i32) -> MoveGenerationData {
        if node_height == 0 {
            MoveGenerationData {
                evaluation: heuristic(&board, bot_colour),
                height: 0,
                bit_move: move_made,
            }
        } else {
            let legal_moves = board.generate_moves();

            if legal_moves.len() == 0 {
                return MoveGenerationData {
                    evaluation: heuristic(&board, bot_colour),
                    height: node_height,
                    bit_move: move_made,
                }
            }

            // If it is the bot's move, we pass up the maximum score, otherwise we pass up the minimum:
            if bot_colour == board.turn() {  
                let mut value = MoveGenerationData::worst_evaluation();

                for mv in legal_moves.iter() {
                    board.apply_move(*mv);
                    
                    // We only want to propagate the moves from the original vector of legal_moves.
                    // If we are at the first iteration, we progagate the move we just made.
                    // If not, then we propagate our parent's move.
                    let move_made = if node_height == root_height {*mv} else {move_made};

                    value = std::cmp::max(value, self.search(board, bot_colour, move_made, node_height-1, root_height, alpha, beta));
                    
                    if value.evaluation > beta {
                        board.undo_move();
                        break;
                    }

                    alpha = i32::max(alpha,value.evaluation);

                    board.undo_move();
                }
                return value
            } else {
                let mut value = MoveGenerationData::best_evaluation();

                for mv in legal_moves.iter() {
                    board.apply_move(*mv);

                    // We only want to propagate the moves from the original vector of legal_moves.
                    // If we are at the first iteration, we progagate the move we just made.
                    // If not, then we propagate our parent's move.
                    let move_made = if node_height == root_height {*mv} else {move_made};
                    
                    value = std::cmp::min(value, self.search(board, bot_colour, move_made, node_height-1, root_height, alpha, beta));

                    if value.evaluation < alpha {
                        board.undo_move();
                        break;
                    }

                    beta = i32::min(beta,value.evaluation);

                    board.undo_move();
                }
                return value
            }
        }
    }

    // fn apply_heuristic(&self, bit_move: BitMove, bot_colour: Player) -> i32 {
    //     let mut cloned_board = self.board.clone();
    //     cloned_board.apply_move(bit_move);
    //     heuristic(&cloned_board, bot_colour)
    // }
}
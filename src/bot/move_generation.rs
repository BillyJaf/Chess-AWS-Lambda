use std::{i32};
use pleco::{BitMove, Board, Player};
use crate::bot::{heuristics::piece_count_heuristic, types::{ BestMove, MoveGenerationData }, utils::is_game_over};

pub fn generate_best_move_recursive(mut board: Board, search_depth: i32) -> BestMove {

    if is_game_over(&board) {
        return BestMove { 
            uci_move: None,
            resulting_board: None,
        };
    }

    let bot_colour = board.turn();

    // Note that there must be atleast one legal move, since we checked if the game ended already.
    let legal_moves = board.generate_moves();

    let move_gen = recursive_generation(&mut board, bot_colour, *legal_moves.iter().next().unwrap(), search_depth, search_depth, i32::MIN, i32::MAX);

    board.apply_move(move_gen.bit_move);

    BestMove { 
        uci_move: Some(move_gen.bit_move.stringify()),
        resulting_board: Some(board),
    }
}

fn recursive_generation(board: &mut Board, bot_colour: Player, move_made: BitMove, current_height: i32, max_height: i32, mut alpha: i32, mut beta: i32) -> MoveGenerationData {
    if current_height == 0 {
        MoveGenerationData {
            evaluation: piece_count_heuristic(&board, bot_colour),
            height: 0,
            bit_move: move_made,
        }
    } else {
        let legal_moves = board.generate_moves();

        if legal_moves.len() == 0 {
            return MoveGenerationData {
                evaluation: piece_count_heuristic(&board, bot_colour),
                height: current_height,
                bit_move: move_made,
            }
        }

        // If it is our move, we pass up the maximum score, otherwise we pass up the minimum:
        if bot_colour == board.turn() {  
            let mut value = MoveGenerationData {
                evaluation: i32::MIN,
                height: max_height,
                bit_move: move_made,
            };

            for mv in legal_moves.iter() {
                board.apply_move(*mv);
                
                // We only want to propagate the moves from the original vector of legal_moves
                // We know that there will be legal_moves in the first iteration at least, hence we only do it here.
                if current_height == max_height {
                    value = std::cmp::max(value, recursive_generation(board, bot_colour, *mv, current_height-1, max_height, alpha, beta));
                    
                    if value.evaluation > beta {
                        board.undo_move();
                        break;
                    }

                    alpha = i32::max(alpha,value.evaluation);

                } else {
                    value = std::cmp::max(value, recursive_generation(board, bot_colour, move_made, current_height-1, max_height, alpha, beta));

                    if value.evaluation > beta {
                        board.undo_move();
                        break;
                    }

                    alpha = i32::max(alpha,value.evaluation);
                }
                board.undo_move();
            }
            return value
        } else {
            let mut value = MoveGenerationData {
                evaluation: i32::MAX,
                height: 0,
                bit_move: move_made,
            };

            for mv in legal_moves.iter() {
                board.apply_move(*mv);
                
                // We only want to propagate the moves from the original vector of legal_moves
                // We know that there will be legal_moves in the first iteration at least, hence we only do it here.
                if current_height == max_height {
                    value = std::cmp::min(value, recursive_generation(board, bot_colour, *mv, current_height-1, max_height, alpha, beta));

                    if value.evaluation < alpha {
                        board.undo_move();
                        break;
                    }

                    beta = i32::min(beta,value.evaluation);
                } else {
                    value = std::cmp::min(value, recursive_generation(board, bot_colour, move_made, current_height-1, max_height, alpha, beta));

                    if value.evaluation < alpha {
                        board.undo_move();
                        break;
                    }

                    beta = i32::min(beta,value.evaluation);
                }
                board.undo_move();
            }
            return value
        }
    }
}
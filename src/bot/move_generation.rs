use pleco::{BitMove, Board, Player};
use crate::bot::{heuristics::piece_count_heuristic, types::{ BestMove, MoveGenerationData }, utils::is_game_over};

pub fn generate_best_move(mut board: Board, search_depth: i32) -> BestMove {

    if is_game_over(&board) {
        return BestMove { 
            uci_move: None,
            resulting_board: None,
        };
    }

    let bot_colour = board.turn();

    let move_gen = generate_tree(&mut board, bot_colour, BitMove::null(), search_depth, search_depth);

    board.apply_move(move_gen.bit_move);

    BestMove { 
        uci_move: Some(move_gen.bit_move.stringify()),
        resulting_board: Some(board),
    }
}

// Returns a tuple (evaluation, distance from leaves)
fn generate_tree(board: &mut Board, bot_colour: Player, move_made: BitMove, current_height: i32, max_height: i32) -> MoveGenerationData {
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

        let mut evaluations: Vec<MoveGenerationData> = Vec::with_capacity(legal_moves.len());

        for mv in legal_moves.iter() {
            board.apply_move(*mv);
            
            // We only want to propagate the moves from the original vector of legal_moves
            // We know that there will be legal_moves in the first iteration at least, hence we only do it here.
            if current_height == max_height {
                evaluations.push(generate_tree(board, bot_colour, *mv, current_height-1, max_height));
            } else {
                evaluations.push(generate_tree(board, bot_colour, move_made, current_height-1, max_height));
            }

            board.undo_move();
        }
        // If it is our move, we pass up the maximum score, otherwise we pass up the minimum:
        if bot_colour == board.turn() {  
            evaluations.iter().max().unwrap().clone()
        } else {
            evaluations.iter().min().unwrap().clone()
        }
    }
}
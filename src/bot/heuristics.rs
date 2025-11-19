use pleco::{ Board, Player};

pub fn piece_count_heuristic(board: &Board, bot_colour: Player) -> i32 {
    let mut evaluation = 0;

    if board.checkmate() {
        if board.turn() == bot_colour {
            return i32::MIN;
        } else {
            return i32::MAX;
        }
    }

    if board.stalemate() {
        return evaluation
    }


    if let Some(fen_board) = board.fen().split_whitespace().next() {
        for piece in fen_board.chars() {
            match piece {
                'p' => evaluation += piece_value(1, Player::Black, bot_colour),
                'n' => evaluation += piece_value(3, Player::Black, bot_colour),
                'b' => evaluation += piece_value(3, Player::Black, bot_colour),
                'r' => evaluation += piece_value(5, Player::Black, bot_colour),
                'q' => evaluation += piece_value(9, Player::Black, bot_colour),

                'P' => evaluation += piece_value(1, Player::White, bot_colour),
                'N' => evaluation += piece_value(3, Player::White, bot_colour),
                'B' => evaluation += piece_value(3, Player::White, bot_colour),
                'R' => evaluation += piece_value(5, Player::White, bot_colour),
                'Q' => evaluation += piece_value(9, Player::White, bot_colour),
                
                _ => continue,
            }
        }
    }

    evaluation
}

pub fn piece_count_heuristic_from_fen(fen: &str, bot_colour: Player) -> i32 {
    piece_count_heuristic(&Board::from_fen(fen).unwrap(), bot_colour)
}

fn piece_value(value: i32, piece_colour: Player, bot_colour: Player) -> i32 {
    if piece_colour == bot_colour {
        value
    } else {
        -value
    }
}
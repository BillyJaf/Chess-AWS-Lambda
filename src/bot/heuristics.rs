use pleco::{ Board, Piece, Player, SQ};

pub fn heuristic(board: &Board, bot_colour: Player) -> i32 {
    if board.checkmate() {
        if board.turn() == bot_colour {
            return i32::MIN;
        } else {
            return i32::MAX;
        }
    }

    if board.stalemate() {
        return 0
    }

    // Count the pieces on board by traditional evaluation.
    // This is used to determine if the game is early or end (for king evaluation).
    // We discard kings from this evaluation.
    let mut traditional_piece_value = 0;

    for i in 0..64 {
        match board.piece_at_sq(SQ(i)) {
            Piece::WhitePawn => traditional_piece_value += 1,
            Piece::WhiteKnight => traditional_piece_value += 3,
            Piece::WhiteBishop => traditional_piece_value += 3,
            Piece::WhiteRook => traditional_piece_value += 5,
            Piece::WhiteQueen => traditional_piece_value += 9,

            Piece::BlackPawn => traditional_piece_value += 1,
            Piece::BlackKnight => traditional_piece_value += 3,
            Piece::BlackBishop => traditional_piece_value += 3,
            Piece::BlackRook => traditional_piece_value += 5,
            Piece::BlackQueen => traditional_piece_value += 9,

            _ => continue
        }
    }

    // Threshold for the game to be considered endgame.
    // This is subject to scrutiny / changing.
    let is_endgame = traditional_piece_value <= 30;

    let mut evaluation = 0;

    for i in 0..64 {
        match board.piece_at_sq(SQ(i)) {
            Piece::WhitePawn => evaluation += pawn_value(Player::White, bot_colour, i),
            Piece::WhiteKnight => evaluation += knight_value(Player::White, bot_colour, i),
            Piece::WhiteBishop => evaluation += bishop_value(Player::White, bot_colour, i),
            Piece::WhiteRook => evaluation += rook_value(Player::White, bot_colour, i),
            Piece::WhiteQueen => evaluation += queen_value(Player::White, bot_colour, i),
            Piece::WhiteKing => {
                if is_endgame {
                    evaluation += king_end_value(Player::White, bot_colour, i)
                } else {
                    evaluation += king_early_value(Player::White, bot_colour, i)
                }
            },

            Piece::BlackPawn => evaluation += pawn_value(Player::Black, bot_colour, i),
            Piece::BlackKnight => evaluation += knight_value(Player::Black, bot_colour, i),
            Piece::BlackBishop => evaluation += bishop_value(Player::Black, bot_colour, i),
            Piece::BlackRook => evaluation += rook_value(Player::Black, bot_colour, i),
            Piece::BlackQueen => evaluation += queen_value(Player::Black, bot_colour, i),
            Piece::BlackKing => {
                if is_endgame {
                    evaluation += king_end_value(Player::Black, bot_colour, i)
                } else {
                    evaluation += king_early_value(Player::Black, bot_colour, i)
                }
            },

            _ => continue
        }
    }
    evaluation
}

fn pawn_value(piece_colour: Player, bot_colour: Player, square_index: u8) -> i32 {

    let pawn_value = 100;
    let pawn_square_evaluation: i32;

    if bot_colour == Player::Black {
        pawn_square_evaluation = BLACK_PAWN_EVALUATION[square_index as usize] as i32
    } else {
        pawn_square_evaluation = WHITE_PAWN_EVALUATION[square_index as usize] as i32
    }

    if piece_colour == bot_colour {
        pawn_value + pawn_square_evaluation
    } else {
        -(pawn_value + pawn_square_evaluation)
    }
}

fn knight_value(piece_colour: Player, bot_colour: Player, square_index: u8) -> i32 {

    let knight_value = 320;
    let knight_square_evaluation: i32;

    if bot_colour == Player::Black {
        knight_square_evaluation = BLACK_KNIGHT_EVALUATION[square_index as usize] as i32
    } else {
        knight_square_evaluation = WHITE_KNIGHT_EVALUATION[square_index as usize] as i32
    }

    if piece_colour == bot_colour {
        knight_value + knight_square_evaluation
    } else {
        -(knight_value + knight_square_evaluation)
    }
}

fn bishop_value(piece_colour: Player, bot_colour: Player, square_index: u8) -> i32 {

    let bishop_value = 330;
    let bishop_square_evaluation: i32;

    if bot_colour == Player::Black {
        bishop_square_evaluation = BLACK_BISHOP_EVALUATION[square_index as usize] as i32
    } else {
        bishop_square_evaluation = WHITE_BISHOP_EVALUATION[square_index as usize] as i32
    }

    if piece_colour == bot_colour {
        bishop_value + bishop_square_evaluation
    } else {
        -(bishop_value + bishop_square_evaluation)
    }
}

fn rook_value(piece_colour: Player, bot_colour: Player, square_index: u8) -> i32 {

    let rook_value = 500;
    let rook_square_evaluation: i32;

    if bot_colour == Player::Black {
        rook_square_evaluation = BLACK_ROOK_EVALUATION[square_index as usize] as i32
    } else {
        rook_square_evaluation = WHITE_ROOK_EVALUATION[square_index as usize] as i32
    }

    if piece_colour == bot_colour {
        rook_value + rook_square_evaluation
    } else {
        -(rook_value + rook_square_evaluation)
    }
}

fn queen_value(piece_colour: Player, bot_colour: Player, square_index: u8) -> i32 {

    let queen_value = 900;
    let queen_square_evaluation: i32;

    if bot_colour == Player::Black {
        queen_square_evaluation = BLACK_QUEEN_EVALUATION[square_index as usize] as i32
    } else {
        queen_square_evaluation = WHITE_QUEEN_EVALUATION[square_index as usize] as i32
    }

    if piece_colour == bot_colour {
        queen_value + queen_square_evaluation
    } else {
        -(queen_value + queen_square_evaluation)
    }
}

fn king_early_value(piece_colour: Player, bot_colour: Player, square_index: u8) -> i32 {

    let king_value = 20000;
    let king_square_evaluation: i32;

    if bot_colour == Player::Black {
        king_square_evaluation = BLACK_KING_EARLY_EVALUATION[square_index as usize] as i32
    } else {
        king_square_evaluation = WHITE_KING_EARLY_EVALUATION[square_index as usize] as i32
    }

    if piece_colour == bot_colour {
        king_value + king_square_evaluation
    } else {
        -(king_value + king_square_evaluation)
    }
}

fn king_end_value(piece_colour: Player, bot_colour: Player, square_index: u8) -> i32 {

    let king_value = 20000;
    let king_square_evaluation: i32;

    if bot_colour == Player::Black {
        king_square_evaluation = BLACK_KING_END_EVALUATION[square_index as usize] as i32
    } else {
        king_square_evaluation = WHITE_KING_END_EVALUATION[square_index as usize] as i32
    }

    if piece_colour == bot_colour {
        king_value + king_square_evaluation
    } else {
        -(king_value + king_square_evaluation)
    }
}

// The following value arrays are from: https://www.chessprogramming.org/Simplified_Evaluation_Function
// Note, the rows in the board are setup in reverse order, but not the columns.
// i.e. the first eight entries in the array are the last eight from the website (the columsn remain unchanged).
//
// The setup of the board is:           A1,  B1,  C1,  D1,  E1,  F1,  G1,  H1, 
//                                      A2,  B2,  C2,  D2,  E2,  F2,  G2,  H2,
//                                      A3,  B3,  C3,  D3,  E3,  F3,  G3,  H3,
//                                      A4,  B4,  C4,  D4,  E4,  F4,  G4,  H4,
//                                      A5,  B5,  C5,  D5,  E5,  F5,  G5,  H5,
//                                      A6,  B6,  C6,  D6,  E6,  F6,  G6,  H6,
//                                      A7,  B7,  C7,  D7,  E7,  F7,  G7,  H7,
//                                      A8,  B8,  C8,  D8,  E8,  F8,  G8,  H8,
//
// The reasoning for this decision is that Pleco indexes square A1 = 0, B1 = 1...
// If the board was setup normally, then the square A8 would be indexed with 0.
// This arrangement allows for Pleco square indexes to be used into the evaluation table.
// This is true for every evaluation. The black evaluation is the reverse of the white evaluation.

static WHITE_PAWN_EVALUATION: [i8; 64] = [ 0,   0,   0,   0,   0,   0,   0,   0,
                                           5,  10,  10, -20, -20,  10,  10,   5,
                                           5,  -5, -10,   0,   0, -10,  -5,   5,
                                           0,   0,   0,  20,  20,   0,   0,   0,
                                           5,   5,  10,  25,  25,  10,   5,   5,
                                          10,  10,  20,  30,  30,  20,  10,  10,
                                          50,  50,  50,  50,  50,  50,  50,  50,
                                           0,   0,   0,   0,   0,   0,   0,   0];

static BLACK_PAWN_EVALUATION: [i8; 64] = [ 0,   0,   0,   0,   0,   0,   0,   0,
                                          50,  50,  50,  50,  50,  50,  50,  50,
                                          10,  10,  20,  30,  30,  20,  10,  10,
                                           5,   5,  10,  25,  25,  10,   5,   5,
                                           0,   0,   0,  20,  20,   0,   0,   0,
                                           5,  -5, -10,   0,   0, -10,  -5,   5,
                                           5,  10,  10, -20, -20,  10,  10,   5,
                                           0,   0,   0,   0,   0,   0,   0,   0];



static WHITE_KNIGHT_EVALUATION: [i8; 64] = [-50, -40, -30, -30, -30, -30, -40, -50,
                                            -40, -20,   0,   5,   5,   0, -20, -40,
                                            -30,   5,  10,  15,  15,  10,   5, -30,
                                            -30,   0,  15,  20,  20,  15,   0, -30,
                                            -30,   5,  15,  20,  20,  15,   5, -30,
                                            -30,   0,  10,  15,  15,  10,   0, -30,
                                            -40, -20,   0,   0,   0,   0, -20, -40,
                                            -50, -40, -30, -30, -30, -30, -40, -50];

static BLACK_KNIGHT_EVALUATION: [i8; 64] = [-50, -40, -30, -30, -30, -30, -40, -50,
                                            -40, -20,   0,   0,   0,   0, -20, -40,
                                            -30,   0,  10,  15,  15,  10,   0, -30,
                                            -30,   5,  15,  20,  20,  15,   5, -30,
                                            -30,   0,  15,  20,  20,  15,   0, -30,
                                            -30,   5,  10,  15,  15,  10,   5, -30,
                                            -40, -20,   0,   5,   5,   0, -20, -40,
                                            -50, -40, -30, -30, -30, -30, -40, -50];



static WHITE_BISHOP_EVALUATION: [i8; 64] = [-20, -10, -10, -10, -10, -10, -10, -20,
                                            -10,   5,   0,   0,   0,   0,   5, -10,
                                            -10,  10,  10,  10,  10,  10,  10, -10,
                                            -10,   0,  10,  10,  10,  10,   0, -10,
                                            -10,   5,   5,  10,  10,   5,   5, -10,
                                            -10,   0,   5,  10,  10,   5,   0, -10,
                                            -10,   0,   0,   0,   0,   0,   0, -10,
                                            -20, -10, -10, -10, -10, -10, -10, -20];

static BLACK_BISHOP_EVALUATION: [i8; 64] = [-20, -10, -10, -10, -10, -10, -10, -20,
                                            -10,   0,   0,   0,   0,   0,   0, -10,
                                            -10,   0,   5,  10,  10,   5,   0, -10,
                                            -10,   5,   5,  10,  10,   5,   5, -10,
                                            -10,   0,  10,  10,  10,  10,   0, -10,
                                            -10,  10,  10,  10,  10,  10,  10, -10,
                                            -10,   5,   0,   0,   0,   0,   5, -10,
                                            -20, -10, -10, -10, -10, -10, -10, -20];



static WHITE_ROOK_EVALUATION: [i8; 64] = [  0,  0,  0,  5,  5,  0,  0,  0,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                            5, 10, 10, 10, 10, 10, 10,  5,
                                            0,  0,  0,  0,  0,  0,  0,  0];

static BLACK_ROOK_EVALUATION: [i8; 64] = [  0,  0,  0,  0,  0,  0,  0,  0,
                                            5, 10, 10, 10, 10, 10, 10,  5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                           -5,  0,  0,  0,  0,  0,  0, -5,
                                            0,  0,  0,  5,  5,  0,  0,  0];



static WHITE_QUEEN_EVALUATION: [i8; 64] = [-20, -10, -10, -5, -5, -10, -10, -20,
                                           -10,   0,   5,  0,  0,   0,   0, -10,
                                           -10,   5,   5,  5,  5,   5,   0, -10,
                                             0,   0,   5,  5,  5,   5,   0,  -5,
                                            -5,   0,   5,  5,  5,   5,   0,  -5,
                                           -10,   0,   5,  5,  5,   5,   0, -10,
                                           -10,   0,   0,  0,  0,   0,   0, -10,
                                           -20, -10, -10, -5, -5, -10, -10, -20];

static BLACK_QUEEN_EVALUATION: [i8; 64] = [-20, -10, -10, -5, -5, -10, -10, -20,
                                           -10,   0,   0,  0,  0,   0,   0, -10,
                                           -10,   0,   5,  5,  5,   5,   0, -10,
                                            -5,   0,   5,  5,  5,   5,   0,  -5,
                                             0,   0,   5,  5,  5,   5,   0,  -5,
                                           -10,   5,   5,  5,  5,   5,   0, -10,
                                           -10,   0,   5,  0,  0,   0,   0, -10,
                                           -20, -10, -10, -5, -5, -10, -10, -20];



static WHITE_KING_EARLY_EVALUATION: [i8; 64] =  [20, 30, 10,  0,  0, 10, 30, 20,
                                                 20, 20,  0,  0,  0,  0, 20, 20,
                                                -10,-20,-20,-20,-20,-20,-20,-10,
                                                -20,-30,-30,-40,-40,-30,-30,-20,
                                                -30,-40,-40,-50,-50,-40,-40,-30,
                                                -30,-40,-40,-50,-50,-40,-40,-30,
                                                -30,-40,-40,-50,-50,-40,-40,-30,
                                                -30,-40,-40,-50,-50,-40,-40,-30];

static BLACK_KING_EARLY_EVALUATION: [i8; 64] = [-30,-40,-40,-50,-50,-40,-40,-30,
                                                -30,-40,-40,-50,-50,-40,-40,-30,
                                                -30,-40,-40,-50,-50,-40,-40,-30,
                                                -30,-40,-40,-50,-50,-40,-40,-30,
                                                -20,-30,-30,-40,-40,-30,-30,-20,
                                                -10,-20,-20,-20,-20,-20,-20,-10,
                                                 20, 20,  0,  0,  0,  0, 20, 20,
                                                 20, 30, 10,  0,  0, 10, 30, 20];

static WHITE_KING_END_EVALUATION: [i8; 64] = [-50,-30,-30,-30,-30,-30,-30,-50,
                                              -30,-30,  0,  0,  0,  0,-30,-30,
                                              -30,-10, 20, 30, 30, 20,-10,-30,
                                              -30,-10, 30, 40, 40, 30,-10,-30,
                                              -30,-10, 30, 40, 40, 30,-10,-30,
                                              -30,-10, 20, 30, 30, 20,-10,-30,
                                              -30,-20,-10,  0,  0,-10,-20,-30,
                                              -50,-40,-30,-20,-20,-30,-40,-50];

static BLACK_KING_END_EVALUATION: [i8; 64] = [-50,-40,-30,-20,-20,-30,-40,-50,
                                              -30,-20,-10,  0,  0,-10,-20,-30,
                                              -30,-10, 20, 30, 30, 20,-10,-30,
                                              -30,-10, 30, 40, 40, 30,-10,-30,
                                              -30,-10, 30, 40, 40, 30,-10,-30,
                                              -30,-10, 20, 30, 30, 20,-10,-30,
                                              -30,-30,  0,  0,  0,  0,-30,-30,
                                              -50,-30,-30,-30,-30,-30,-30,-50];
use std::{cmp::Ordering, u8};
use pleco::{BitMove, Board};
use serde::{Deserialize, Serialize};

/// Represents an evaluation score along with its height in the game tree.
/// Height refers to the number of levels a node is from the deepest leaf.
/// 
/// - The first `i32` is the evaluation score.
/// - The second `i32` is the height of the node in the search tree.
/// - The `BitMove` is the best move to make after consideration.
#[derive(Debug, Eq, Clone, Copy, PartialEq)]
pub struct MoveGenerationData {
    pub evaluation: i32,
    pub height: u8,
    pub bit_move: BitMove
}

impl MoveGenerationData {
    pub fn worst_evaluation() -> Self {
        Self {
            evaluation: i32::MIN,
            height: u8::MAX,
            bit_move: BitMove::null(),
        }
    }

    pub fn best_evaluation() -> Self {
        Self {
            evaluation: i32::MAX,
            height: 0,
            bit_move: BitMove::null(),
        }
    }
}

/// Order first by the evaluation.
/// If the evaluations are equal, order second by the height.
impl Ord for MoveGenerationData {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.evaluation.cmp(&other.evaluation) {
            Ordering::Equal => self.height.cmp(&other.height),
            ord => ord,
        }
    }
}

impl PartialOrd for MoveGenerationData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct BestMove {
    pub uci_move: String,
    pub san_move: String,
    pub resulting_board: Board,
}

#[derive(Serialize)]
pub enum GameOver {
    White,
    Black,
    Stalemate,
}

#[derive(Serialize)]
pub struct ResultingGameState {
    pub uci_move: String,
    pub san_move: String,
    pub resulting_fen: String,
    pub game_over: Option<GameOver>,
}

#[derive(Serialize)]
pub struct BestMoveResponse {
    pub game_over: Option<GameOver>,
    pub uci_move: String,
    pub san_move: String,
    pub resulting_fen: String,
    pub resulting_legal_moves: Vec<ResultingGameState>
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct LegalMoves {
    pub game_over: Option<GameOver>,
    pub legal_moves: Vec<ResultingGameState>,
}

#[derive(Serialize)]
pub struct ValidateFenResponse {
    pub valid: bool,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseError {
    pub error: String,
}

#[derive(Deserialize)]
pub struct FenInput {
    pub fen: String,
}
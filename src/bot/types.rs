use std::cmp::Ordering;
use pleco::{BitMove, Board};

/// Represents an evaluation score along with its height in the game tree.
/// Height refers to the number of levels a node is from the deepest leaf.
/// 
/// - The first `i32` is the evaluation score.
/// - The second `i32` is the height of the node in the search tree.
/// - The `BitMove` is the best move to make after consideration.
#[derive(Debug, Eq, Clone, Copy, PartialEq)]
pub struct MoveGenerationData {
    pub evaluation: i32,
    pub height: i32,
    pub bit_move: BitMove
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

impl MoveGenerationData {
    pub fn max_move_gen(&self, other: &Self) -> Self {
        let bit_move: BitMove;
        if self.height > other.height {
            bit_move = self.bit_move;
        } else {
            bit_move = other.bit_move;
        }

        Self {
            bit_move,
            ..std::cmp::max(*self,*other)
        }
    }

    pub fn min_move_gen(&self, other: &Self) -> Self {
        let bit_move: BitMove;
        if self.height > other.height {
            bit_move = self.bit_move;
        } else {
            bit_move = other.bit_move;
        }

        Self {
            bit_move,
            ..std::cmp::min(*self,*other)
        }
    }
}

pub struct BestMove {
    pub uci_move: Option<String>,
    pub resulting_board: Option<Board>,
}
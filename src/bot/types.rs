use std::cmp::Ordering;
use pleco::{BitMove, Board};
use std::cell::RefCell;
use std::rc::{Rc};

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

// impl Copy for MoveGenerationData {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

pub struct BestMove {
    pub uci_move: Option<String>,
    pub resulting_board: Option<Board>,
}

#[derive(Debug)]
pub struct MoveGenerationNode {
    pub fen: String,
    pub move_made: BitMove,
    pub height: i32,
    pub evaluation: Option<i32>,
    pub children: Vec<Box<MoveGenerationNode>>,
}

impl MoveGenerationNode {
    pub fn new(fen: String, max_height: i32) -> Self {
        Self {
            fen,
            move_made: BitMove::null(),
            height: max_height,
            evaluation: None,
            children: Vec::new(),
        }
    }

    fn new_child(fen: String, parent_height: i32, move_made: BitMove) -> Self {
        Self {
            fen,
            move_made: move_made,
            height: parent_height - 1,
            evaluation: None,
            children: Vec::new(),
        }
    }

    pub fn add_legal_children(node: &mut Self) {
        let mut board = Board::from_fen(&node.fen).unwrap();
        let legal_moves = board.generate_moves();

        for legal_move in legal_moves.iter() {
            board.apply_move(*legal_move);
            node.children.push(Box::new(MoveGenerationNode::new_child(board.fen(), node.height, *legal_move)));
            board.undo_move();
        }
    }
}
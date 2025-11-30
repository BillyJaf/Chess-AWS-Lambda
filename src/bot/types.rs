use std::{cmp::Ordering, collections::HashMap};
use pleco::{BitMove, Board};
use xxhash_rust::xxh3::xxh3_64;

pub struct Engine {
    board: Board,
    transposition_table: TranspositionTable,
}

struct TranspositionTable {
    size: usize,
    map: HashMap<u64, TTE>,
}

struct TTE {
    pub height: u8,
    pub evaluation: i32,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            map: HashMap::with_capacity(size),
        }
    }

    pub fn insert(&mut self, board: &Board, height: u8, evaluation: i32) {
        if self.map.len() >= self.size {
            return // FIX THIS SO THAT IT EVICTS AND THEN ADDS
        }
        self.map.insert(xxh3_64(board.fen().as_bytes()), TTE {
            height,
            evaluation,
        });
    }
}

impl Engine {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            transposition_table: TranspositionTable::new(200_000),
        }
    }
}

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

pub struct BestMove {
    pub uci_move: Option<String>,
    pub resulting_board: Option<Board>,
}
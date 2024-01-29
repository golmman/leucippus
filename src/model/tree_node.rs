use super::board::Board;
use super::board_evaluation::BoardEvaluation;
use super::types::TreeNodeIndex;

#[derive(Debug)]
pub struct TreeNode {
    pub board: Board,
    pub board_hash: u64,
    pub child_indices: Vec<TreeNodeIndex>,
    pub evaluation: BoardEvaluation,
    pub parent_index: Option<TreeNodeIndex>,
    pub score: TreeNodeScore,
}

#[derive(Debug)]
pub struct TreeNodeScore {
    pub draws: u64,
    pub wins_black: u64,
    pub wins_white: u64,
}

impl TreeNode {
    pub fn new(board: Board, parent_index: Option<TreeNodeIndex>) -> Self {
        let board_hash = board.get_hash();
        Self {
            board,
            board_hash,
            child_indices: Vec::new(),
            evaluation: BoardEvaluation::Inconclusive,
            parent_index,
            score: TreeNodeScore {
                draws: 0,
                wins_black: 0,
                wins_white: 0,
            },
        }
    }
}

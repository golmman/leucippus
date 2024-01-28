use super::board::Board;
use super::types::TreeNodeIndex;

#[derive(Debug)]
pub struct TreeNode {
    pub board: Board,
    pub board_hash: u64,
    pub child_indices: Vec<TreeNodeIndex>,
    pub game_over: bool,
    pub parent_index: Option<TreeNodeIndex>,
    pub score: TreeNodeScore,
}

#[derive(Debug)]
pub struct TreeNodeScore {
    pub draws: u64,
    pub losses: u64,
    pub wins: u64,
}

impl TreeNode {
    pub fn new(board: Board, parent_index: Option<TreeNodeIndex>) -> Self {
        let board_hash = board.get_hash();
        Self {
            board,
            board_hash,
            child_indices: Vec::new(),
            game_over: false,
            parent_index,
            score: TreeNodeScore {
                draws: 0,
                losses: 0,
                wins: 0,
            },
        }
    }
}

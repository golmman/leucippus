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
    pub self_index: TreeNodeIndex,
}

#[derive(Debug, PartialEq)]
pub struct TreeNodeScore {
    pub draws: u64,
    pub wins_black: u64,
    pub wins_white: u64,
}

impl TreeNode {
    pub fn is_not_visited(&self) -> bool {
        0 == self.score.draws + self.score.wins_black + self.score.wins_white
    }
}

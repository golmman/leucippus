use super::board::Board;
use super::tree_node::TreeNode;

// TODO: remove?
pub struct SimulationStep {
    pub board: Board,
    pub board_hash: u64,
}

impl From<&TreeNode> for SimulationStep {
    fn from(node: &TreeNode) -> Self {
        Self {
            board: node.board.clone(),
            board_hash: node.board_hash,
        }
    }
}

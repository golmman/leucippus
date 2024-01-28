use super::{board::Board, tree_node::TreeNode};

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

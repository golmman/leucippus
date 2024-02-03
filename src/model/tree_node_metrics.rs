use super::board_evaluation::BoardEvaluation;
use super::r#move::Move;
use super::tree_node::TreeNode;
use super::tree_node::TreeNodeScore;

pub struct TreeNodeMetrics {
    pub score: TreeNodeScore,
    pub last_move: Move,
    pub evaluation: BoardEvaluation,
}

impl From<&TreeNode> for TreeNodeMetrics {
    fn from(node: &TreeNode) -> Self {
        Self {
            score: node.score.clone(),
            last_move: node.last_move,
            evaluation: node.evaluation,
        }
    }
}

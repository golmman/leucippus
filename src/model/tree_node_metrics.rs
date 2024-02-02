use std::cmp::Ordering;

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

pub fn compare_black(
    left: &TreeNodeMetrics,
    right: &TreeNodeMetrics,
) -> Ordering {
    if left.evaluation == BoardEvaluation::WinBlack
        && right.evaluation == BoardEvaluation::WinBlack
    {
        return Ordering::Equal;
    }
    if left.evaluation != BoardEvaluation::WinBlack
        && right.evaluation == BoardEvaluation::WinBlack
    {
        return Ordering::Greater;
    }
    if left.evaluation == BoardEvaluation::WinBlack
        && right.evaluation != BoardEvaluation::WinBlack
    {
        return Ordering::Less;
    }

    right.score.wins_black.cmp(&left.score.wins_black)
}

pub fn compare_white(
    left: &TreeNodeMetrics,
    right: &TreeNodeMetrics,
) -> Ordering {
    if left.evaluation == BoardEvaluation::WinWhite
        && right.evaluation == BoardEvaluation::WinWhite
    {
        return Ordering::Equal;
    }
    if left.evaluation != BoardEvaluation::WinWhite
        && right.evaluation == BoardEvaluation::WinWhite
    {
        return Ordering::Greater;
    }
    if left.evaluation == BoardEvaluation::WinWhite
        && right.evaluation != BoardEvaluation::WinWhite
    {
        return Ordering::Less;
    }

    right.score.wins_white.cmp(&left.score.wins_white)
}

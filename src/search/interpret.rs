use std::cmp::Ordering;

use crate::model::board_evaluation::BoardEvaluation;
use crate::model::color::Color;
use crate::model::tree::Tree;
use crate::model::tree_node_metrics::TreeNodeMetrics;

pub fn get_search_result(tree: &Tree) -> Vec<TreeNodeMetrics> {
    let root_node = tree.get_root();
    let our_color = root_node.our_color;

    let mut metrics: Vec<TreeNodeMetrics> = root_node
        .child_indices
        .iter()
        .map(|c| tree.get_node(*c))
        .map(TreeNodeMetrics::from)
        .collect();

    if our_color == Color::Black {
        metrics.sort_by(compare_black);
    } else {
        metrics.sort_by(compare_white);
    }

    metrics
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

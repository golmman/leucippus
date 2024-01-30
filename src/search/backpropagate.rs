use crate::model::board_evaluation::BoardEvaluation;
use crate::model::simulation_result::SimulationResult;
use crate::model::tree::Tree;
use crate::model::types::TreeNodeIndex;

pub fn backpropagate(
    tree: &mut Tree,
    node_index: TreeNodeIndex,
    simulation_result: SimulationResult,
) {
    let mut node = tree.get_node_mut(node_index);

    if simulation_result.depth == 0 {
        node.evaluation = simulation_result.evaluation;
    }

    loop {
        if let Some(win_color) = node.evaluation.get_win_color() {
            //tree.get_sibling_indices(
            //if win_color != board.active_color
            //    || parent
            //        .children
            //        .iter()
            //        .all(|c| c.evaluation == node.evaluation)
            //{
            //    parent.evaluation = node.evaluation;
            //}
        } else {
            match simulation_result.evaluation {
                BoardEvaluation::Draw => node.score.draws += 1,
                BoardEvaluation::Inconclusive => panic!(),
                BoardEvaluation::WinBlack => node.score.wins_black += 1,
                BoardEvaluation::WinWhite => node.score.wins_white += 1,
            }
        }

        let Some(parent_index) = node.parent_index else {
            return;
        };
        node = tree.get_node_mut(parent_index);
    }
}

#[cfg(test)]
mod test {
    use super::*;
}

use crate::model::board_evaluation::BoardEvaluation;
use crate::model::simulation_result::SimulationResult;
use crate::model::tree::Tree;
use crate::model::types::TreeNodeIndex;

pub fn backpropagate(
    tree: &mut Tree,
    node_index: TreeNodeIndex,
    simulation_result: SimulationResult,
) {
    if simulation_result.depth == 0 {
        tree.get_node_mut(node_index).evaluation = simulation_result.evaluation;
    }

    let pv_indices = get_principal_variation_node_indices(&tree, node_index);

    for n in pv_indices {
        let node = tree.get_node(n);
        if let Some(win_color) = node.evaluation.get_win_color() {
            let node_evaluation = node.evaluation;

            if win_color != node.our_color
                || tree
                    .get_sibling_indices(n)
                    .iter()
                    .all(|s| tree.get_node(*s).evaluation == node_evaluation)
            {
                if let Some(parent) = tree.get_parent_mut(n) {
                    parent.evaluation = node_evaluation;
                }
            }
        }

        let node = tree.get_node_mut(n);
        match simulation_result.evaluation {
            BoardEvaluation::Draw => node.score.draws += 1,
            BoardEvaluation::Inconclusive => panic!(),
            BoardEvaluation::WinBlack => node.score.wins_black += 1,
            BoardEvaluation::WinWhite => node.score.wins_white += 1,
        }
    }
}

fn get_principal_variation_node_indices(
    tree: &Tree,
    node_index: TreeNodeIndex,
) -> Vec<TreeNodeIndex> {
    let mut index = node_index;
    let mut indices = Vec::new();

    loop {
        indices.push(index);
        let Some(parent) = tree.get_node(index).parent_index else {
            return indices;
        };
        index = parent;
    }
}

#[cfg(test)]
mod test {
    use crate::model::board::Board;
    use crate::model::r#move::Move;
    use crate::model::tree_node::TreeNodeScore;

    use super::*;

    #[test]
    fn it_calculates_the_principal_variation_indices() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 1);
        tree.add_node(Board::new(), Move::from_to(0, 0), 2);
        tree.add_node(Board::new(), Move::from_to(0, 0), 2);
        tree.add_node(Board::new(), Move::from_to(0, 0), 3);
        tree.add_node(Board::new(), Move::from_to(0, 0), 5);
        tree.add_node(Board::new(), Move::from_to(0, 0), 6);

        let indices = get_principal_variation_node_indices(&tree, 7);

        assert_eq!(indices, [7, 6, 5, 3, 2, 1, 0]);
    }

    #[test]
    fn it_updates_draw_scores_during_backpropagation() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), Move::from_to(0, 0), 0); // index = 1
        tree.add_node(Board::new(), Move::from_to(0, 0), 1); // index = 2
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 3
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 4
        tree.add_node(Board::new(), Move::from_to(0, 0), 3); // index = 5
        tree.add_node(Board::new(), Move::from_to(0, 0), 5); // index = 6
        let simulation_result = SimulationResult {
            depth: 10,
            evaluation: BoardEvaluation::Draw,
        };

        backpropagate(&mut tree, 6, simulation_result);

        assert_eq!(tree.get_node(0).score, score(1, 0, 0));
        assert_eq!(tree.get_node(1).score, score(1, 0, 0));
        assert_eq!(tree.get_node(2).score, score(1, 0, 0));
        assert_eq!(tree.get_node(3).score, score(1, 0, 0));
        assert_eq!(tree.get_node(4).score, score(0, 0, 0));
        assert_eq!(tree.get_node(5).score, score(1, 0, 0));
        assert_eq!(tree.get_node(6).score, score(1, 0, 0));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::Inconclusive);
    }

    #[test]
    fn it_updates_black_win_scores_during_backpropagation() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), Move::from_to(0, 0), 0); // index = 1
        tree.add_node(Board::new(), Move::from_to(0, 0), 1); // index = 2
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 3
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 4
        tree.add_node(Board::new(), Move::from_to(0, 0), 3); // index = 5
        tree.add_node(Board::new(), Move::from_to(0, 0), 5); // index = 6
        let simulation_result = SimulationResult {
            depth: 10,
            evaluation: BoardEvaluation::WinBlack,
        };

        backpropagate(&mut tree, 6, simulation_result);

        assert_eq!(tree.get_node(0).score, score(0, 1, 0));
        assert_eq!(tree.get_node(1).score, score(0, 1, 0));
        assert_eq!(tree.get_node(2).score, score(0, 1, 0));
        assert_eq!(tree.get_node(3).score, score(0, 1, 0));
        assert_eq!(tree.get_node(4).score, score(0, 0, 0));
        assert_eq!(tree.get_node(5).score, score(0, 1, 0));
        assert_eq!(tree.get_node(6).score, score(0, 1, 0));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::Inconclusive);
    }

    #[test]
    fn it_updates_white_win_scores_during_backpropagation() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), Move::from_to(0, 0), 0); // index = 1
        tree.add_node(Board::new(), Move::from_to(0, 0), 1); // index = 2
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 3
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 4
        tree.add_node(Board::new(), Move::from_to(0, 0), 3); // index = 5
        tree.add_node(Board::new(), Move::from_to(0, 0), 5); // index = 6
        let simulation_result = SimulationResult {
            depth: 10,
            evaluation: BoardEvaluation::WinWhite,
        };

        backpropagate(&mut tree, 6, simulation_result);

        assert_eq!(tree.get_node(0).score, score(0, 0, 1));
        assert_eq!(tree.get_node(1).score, score(0, 0, 1));
        assert_eq!(tree.get_node(2).score, score(0, 0, 1));
        assert_eq!(tree.get_node(3).score, score(0, 0, 1));
        assert_eq!(tree.get_node(4).score, score(0, 0, 0));
        assert_eq!(tree.get_node(5).score, score(0, 0, 1));
        assert_eq!(tree.get_node(6).score, score(0, 0, 1));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::Inconclusive);
    }

    #[test]
    fn it_updates_mixed_score_counts_during_backpropagation() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), Move::from_to(0, 0), 0); // index = 1
        tree.add_node(Board::new(), Move::from_to(0, 0), 1); // index = 2
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 3
        tree.add_node(Board::new(), Move::from_to(0, 0), 2); // index = 4
        tree.add_node(Board::new(), Move::from_to(0, 0), 3); // index = 5
        tree.add_node(Board::new(), Move::from_to(0, 0), 5); // index = 6
        let simulation_result = SimulationResult {
            depth: 10,
            evaluation: BoardEvaluation::WinWhite,
        };

        backpropagate(&mut tree, 6, simulation_result);

        let simulation_result = SimulationResult {
            depth: 10,
            evaluation: BoardEvaluation::WinBlack,
        };

        backpropagate(&mut tree, 4, simulation_result);

        assert_eq!(tree.get_node(0).score, score(0, 1, 1));
        assert_eq!(tree.get_node(1).score, score(0, 1, 1));
        assert_eq!(tree.get_node(2).score, score(0, 1, 1));
        assert_eq!(tree.get_node(3).score, score(0, 0, 1));
        assert_eq!(tree.get_node(4).score, score(0, 1, 0));
        assert_eq!(tree.get_node(5).score, score(0, 0, 1));
        assert_eq!(tree.get_node(6).score, score(0, 0, 1));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::Inconclusive);
    }

    #[test]
    fn it_propagates_a_forced_white_win_up_to_root() {
        // W    B    W    B    W    B
        // 0 -> 1 -> 2 -> 3 -> 5 -> 6
        //            `-> 4
        let mut tree = Tree::new(board_white()); // 0, W
        tree.add_node(board_black(), Move::from_to(0, 0), 0); //         1, B
        tree.add_node(board_white(), Move::from_to(0, 0), 1); //         2, W
        tree.add_node(board_black(), Move::from_to(0, 0), 2); //         3, B
        tree.add_node(board_black(), Move::from_to(0, 0), 2); //         4, B
        tree.add_node(board_white(), Move::from_to(0, 0), 3); //         5, W
        tree.add_node(board_black(), Move::from_to(0, 0), 5); //         6, B
        let simulation_result = SimulationResult {
            depth: 0,
            evaluation: BoardEvaluation::WinWhite,
        };

        backpropagate(&mut tree, 6, simulation_result);

        assert_eq!(tree.get_node(0).score, score(0, 0, 1));
        assert_eq!(tree.get_node(1).score, score(0, 0, 1));
        assert_eq!(tree.get_node(2).score, score(0, 0, 1));
        assert_eq!(tree.get_node(3).score, score(0, 0, 1));
        assert_eq!(tree.get_node(4).score, score(0, 0, 0));
        assert_eq!(tree.get_node(5).score, score(0, 0, 1));
        assert_eq!(tree.get_node(6).score, score(0, 0, 1));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::WinWhite);
    }

    #[test]
    fn it_propagates_a_white_win_up_to_a_black_tree_fork() {
        // W    B    W    B    W    B
        // 0 -> 1 -> 2 -> 4 -> 5 -> 6
        //       `-> 3
        let mut tree = Tree::new(board_white()); // 0, W
        tree.add_node(board_black(), Move::from_to(0, 0), 0); //         1, B
        tree.add_node(board_white(), Move::from_to(0, 0), 1); //         2, W
        tree.add_node(board_white(), Move::from_to(0, 0), 1); //         3, W
        tree.add_node(board_black(), Move::from_to(0, 0), 2); //         4, B
        tree.add_node(board_white(), Move::from_to(0, 0), 4); //         5, W
        tree.add_node(board_black(), Move::from_to(0, 0), 5); //         6, B
        let simulation_result = SimulationResult {
            depth: 0,
            evaluation: BoardEvaluation::WinWhite,
        };

        backpropagate(&mut tree, 6, simulation_result);

        assert_eq!(tree.get_node(0).score, score(0, 0, 1));
        assert_eq!(tree.get_node(1).score, score(0, 0, 1));
        assert_eq!(tree.get_node(2).score, score(0, 0, 1));
        assert_eq!(tree.get_node(3).score, score(0, 0, 0));
        assert_eq!(tree.get_node(4).score, score(0, 0, 1));
        assert_eq!(tree.get_node(5).score, score(0, 0, 1));
        assert_eq!(tree.get_node(6).score, score(0, 0, 1));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::WinWhite);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::WinWhite);
    }

    #[test]
    fn it_propagates_a_black_win_up_to_a_white_tree_fork() {
        // W    B    W    B    W    B
        // 0 -> 1 -> 2 -> 3 -> 5 -> 6
        //            `-> 4
        let mut tree = Tree::new(board_white()); // 0, W
        tree.add_node(board_black(), Move::from_to(0, 0), 0); //         1, B
        tree.add_node(board_white(), Move::from_to(0, 0), 1); //         2, W
        tree.add_node(board_black(), Move::from_to(0, 0), 2); //         3, B
        tree.add_node(board_black(), Move::from_to(0, 0), 2); //         4, B
        tree.add_node(board_white(), Move::from_to(0, 0), 3); //         5, W
        tree.add_node(board_black(), Move::from_to(0, 0), 5); //         6, B
        let simulation_result = SimulationResult {
            depth: 0,
            evaluation: BoardEvaluation::WinBlack,
        };

        backpropagate(&mut tree, 6, simulation_result);

        assert_eq!(tree.get_node(0).score, score(0, 1, 0));
        assert_eq!(tree.get_node(1).score, score(0, 1, 0));
        assert_eq!(tree.get_node(2).score, score(0, 1, 0));
        assert_eq!(tree.get_node(3).score, score(0, 1, 0));
        assert_eq!(tree.get_node(4).score, score(0, 0, 0));
        assert_eq!(tree.get_node(5).score, score(0, 1, 0));
        assert_eq!(tree.get_node(6).score, score(0, 1, 0));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::WinBlack);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::WinBlack);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::WinBlack);
    }

    #[test]
    fn it_propagates_sets_a_draw_leaf_and_propagates_the_draw_score() {
        // W    B    W    B    W    B
        // 0 -> 1 -> 2 -> 3 -> 5 -> 6
        //            `-> 4
        let mut tree = Tree::new(board_white()); // 0, W
        tree.add_node(board_black(), Move::from_to(0, 0), 0); //         1, B
        tree.add_node(board_white(), Move::from_to(0, 0), 1); //         2, W
        tree.add_node(board_black(), Move::from_to(0, 0), 2); //         3, B
        tree.add_node(board_black(), Move::from_to(0, 0), 2); //         4, B
        tree.add_node(board_white(), Move::from_to(0, 0), 3); //         5, W
        tree.add_node(board_black(), Move::from_to(0, 0), 5); //         6, B
        let simulation_result = SimulationResult {
            depth: 0,
            evaluation: BoardEvaluation::Draw,
        };

        backpropagate(&mut tree, 6, simulation_result);

        assert_eq!(tree.get_node(0).score, score(1, 0, 0));
        assert_eq!(tree.get_node(1).score, score(1, 0, 0));
        assert_eq!(tree.get_node(2).score, score(1, 0, 0));
        assert_eq!(tree.get_node(3).score, score(1, 0, 0));
        assert_eq!(tree.get_node(4).score, score(0, 0, 0));
        assert_eq!(tree.get_node(5).score, score(1, 0, 0));
        assert_eq!(tree.get_node(6).score, score(1, 0, 0));

        assert_eq!(tree.get_node(0).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(1).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(2).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(3).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(4).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(5).evaluation, BoardEvaluation::Inconclusive);
        assert_eq!(tree.get_node(6).evaluation, BoardEvaluation::Draw);
    }

    fn score(draws: u64, wins_black: u64, wins_white: u64) -> TreeNodeScore {
        TreeNodeScore {
            draws,
            wins_black,
            wins_white,
        }
    }

    fn board_black() -> Board {
        let mut board = Board::new();
        board.swap_color();
        board
    }

    fn board_white() -> Board {
        Board::new()
    }
}

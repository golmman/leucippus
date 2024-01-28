use std::f64::consts::SQRT_2;

use crate::model::board::Board;
use crate::model::tree::Tree;
use crate::model::tree_node::TreeNode;
use crate::model::tree_node::TreeNodeScore;
use crate::model::types::TreeNodeIndex;
use crate::model::types::TREE_NODE_ROOT_INDEX;

pub fn search(board: Board) {
    let mut tree = Tree::new(board);

    select(&mut tree);
}

fn select(tree: &Tree) -> TreeNodeIndex {
    let mut best_child_index = TREE_NODE_ROOT_INDEX;

    loop {
        let parent = tree.get_node(best_child_index);
        if parent.child_indices.is_empty() {
            return best_child_index;
        }

        let mut best_uct = std::f64::MIN;
        for child_index in &parent.child_indices {
            let child = tree.get_node(*child_index);

            if child.game_over {
                continue;
            }

            if is_not_visited(child) {
                return *child_index;
            }

            let uct = calculate_uct(&child.score, &parent.score);
            if uct > best_uct {
                best_uct = uct;
                best_child_index = *child_index;
            }
        }
    }
}

fn expand(tree: &mut Tree) {
    // expand only moves that have no
}

fn simulate(tree: &mut Tree) {}

fn backpropagate(tree: &mut Tree) {}

fn is_not_visited(node: &TreeNode) -> bool {
    0 == node.score.draws + node.score.losses + node.score.wins
}

/// Calculates the upper confidence bound.
/// See:
/// https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
/// https://www.chessprogramming.org/UCT
fn calculate_uct(
    child_score: &TreeNodeScore,
    parent_score: &TreeNodeScore,
) -> f64 {
    let child_visits =
        (child_score.draws + child_score.losses + child_score.wins) as f64;
    let parent_visits =
        (parent_score.draws + parent_score.losses + parent_score.wins) as f64;
    let child_win_ratio = (child_score.wins as f64) / child_visits;

    child_win_ratio + SQRT_2 * (parent_visits.ln() / child_visits).sqrt()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_selects_the_root_not_in_an_otherwise_empty_tree() {
        let tree = Tree::new(Board::new());
        assert_eq!(select(&tree), 0);
    }

    #[test]
    fn it_selects_unvisited_nodes_first() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);

        tree.get_node_mut(0).score.wins = 1;
        tree.get_node_mut(0).score.losses = 1;
        tree.get_node_mut(1).score.wins = 1;
        tree.get_node_mut(3).score.losses = 1;

        assert_eq!(select(&tree), 2);
    }

    #[test]
    fn it_selects_the_node_with_the_highest_uct() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);

        tree.get_node_mut(0).score.wins = 1;
        tree.get_node_mut(0).score.losses = 2;

        tree.get_node_mut(1).score.losses = 1;
        tree.get_node_mut(2).score.wins = 1;
        tree.get_node_mut(3).score.losses = 1;

        assert_eq!(select(&tree), 2);
    }

    #[test]
    fn it_selects_the_nodes_which_are_not_game_over() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);

        tree.get_node_mut(0).score.wins = 1;
        tree.get_node_mut(0).score.losses = 2;

        tree.get_node_mut(1).score.losses = 1;
        tree.get_node_mut(2).score.wins = 1;
        tree.get_node_mut(2).game_over = true;
        tree.get_node_mut(3).score.losses = 1;

        assert_eq!(select(&tree), 1);
    }
}

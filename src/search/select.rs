use crate::model::tree::Tree;
use crate::model::types::TreeNodeIndex;
use crate::model::types::TREE_NODE_ROOT_INDEX;

pub fn select(tree: &Tree) -> TreeNodeIndex {
    let mut best_child_index = TREE_NODE_ROOT_INDEX;

    loop {
        let parent = tree.get_node(best_child_index);
        if parent.child_indices.is_empty() {
            return best_child_index;
        }

        let mut best_uct = std::f64::MIN;
        for child_index in &parent.child_indices {
            let child = tree.get_node(*child_index);

            if child.evaluation.is_conclusive() {
                continue;
            }

            if child.is_not_visited() {
                return *child_index;
            }

            let uct = tree.calculate_uct(*child_index);
            if uct > best_uct {
                best_uct = uct;
                best_child_index = *child_index;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::board::Board;
    use crate::model::board_evaluation::BoardEvaluation;

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

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 1;
        tree.get_node_mut(1).score.wins_white = 1;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree), 2);
    }

    #[test]
    fn it_selects_the_node_with_the_highest_uct() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 2;

        tree.get_node_mut(1).score.wins_black = 1;
        tree.get_node_mut(2).score.wins_white = 1;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree), 2);
    }

    #[test]
    fn it_selects_nodes_whose_evaluation_is_inconclusive() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 2;

        tree.get_node_mut(1).score.wins_black = 1;
        tree.get_node_mut(2).score.wins_white = 1;
        tree.get_node_mut(2).evaluation = BoardEvaluation::Draw;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree), 1);
    }
}

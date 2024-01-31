use crate::common::random::Random;
use crate::model::tree::Tree;
use crate::model::types::TreeNodeIndex;
use crate::model::types::TREE_NODE_ROOT_INDEX;

pub fn select(tree: &Tree, random: &mut Random) -> TreeNodeIndex {
    let mut best_node_indices = vec![TREE_NODE_ROOT_INDEX];

    loop {
        let random_best_node_index = *random
            .pick_element(&best_node_indices)
            .expect("the list of best nodes must never be empty");

        let parent = tree.get_node(random_best_node_index);
        if parent.child_indices.is_empty() {
            return random_best_node_index;
        }

        let mut best_uct = std::u32::MIN;
        for node_index in &parent.child_indices {
            let node = tree.get_node(*node_index);

            if node.evaluation.is_conclusive() {
                continue;
                // TODO: if win_color != eval.get_win -> choose this node!?
            }

            let uct = tree.calculate_uct(*node_index);
            if uct == best_uct {
                best_node_indices.push(*node_index);
            } else if uct > best_uct {
                best_uct = uct;
                best_node_indices = vec![*node_index];
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::board::Board;
    use crate::model::board_evaluation::BoardEvaluation;
    use crate::model::r#move::Move;

    use super::*;

    #[test]
    fn it_selects_the_root_not_in_an_otherwise_empty_tree() {
        let tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);
        assert_eq!(select(&tree, &mut random), 0);
    }

    #[test]
    fn it_selects_unvisited_nodes_first() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 1;
        tree.get_node_mut(1).score.wins_white = 1;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree, &mut random), 2);
    }

    #[test]
    fn it_selects_the_node_with_the_highest_uct() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 2;

        tree.get_node_mut(1).score.wins_black = 1;
        tree.get_node_mut(2).score.wins_white = 1;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree, &mut random), 2);
    }

    #[test]
    fn it_selects_nodes_whose_evaluation_is_inconclusive() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 2;

        tree.get_node_mut(1).score.wins_black = 1;
        tree.get_node_mut(2).score.wins_white = 1;
        tree.get_node_mut(2).evaluation = BoardEvaluation::Draw;
        tree.get_node_mut(3).score.wins_black = 1;

        assert!(select(&tree, &mut random) != 2);
        assert!(select(&tree, &mut random) != 2);
        assert!(select(&tree, &mut random) != 2);
        assert!(select(&tree, &mut random) != 2);
        assert!(select(&tree, &mut random) != 2);
        assert!(select(&tree, &mut random) != 2);
        assert!(select(&tree, &mut random) != 2);
    }
}

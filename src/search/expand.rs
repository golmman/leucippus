use crate::model::tree::Tree;
use crate::model::types::TreeNodeIndex;
use crate::move_generator::legal_moves::generate_moves;
use crate::move_generator::make_move::make_move;

pub fn expand(tree: &mut Tree, node_index: TreeNodeIndex) -> usize {
    let node = tree.get_node_mut(node_index);

    if node.is_not_visited() {
        return node_index;
    }

    debug_assert!(node.child_indices.is_empty());

    // TODO: why is it necessary to have this extra clone?
    // The borrow checker complains otherwise...
    let new_board = node.board.clone();

    let moves = generate_moves(&mut node.board);
    for m in moves {
        let mut new_board2 = new_board.clone();
        make_move(&mut new_board2, &m);
        tree.add_node(new_board2, node_index);
    }

    tree.get_size() - 1
}

#[cfg(test)]
mod test {
    use crate::model::board::Board;

    use super::*;

    #[test]
    fn it_does_not_expand_a_node_which_has_no_visits() {
        let mut tree = Tree::new(Board::new());
        assert_eq!(expand(&mut tree, 0), 0);
        assert_eq!(tree.get_size(), 1);
    }

    #[test]
    #[should_panic]
    fn it_panics_when_trying_to_expand_a_node_with_children() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), 0);
        tree.get_node_mut(0).score.wins_white = 1;
        expand(&mut tree, 0);
    }

    #[test]
    fn it_expands_a_node_with_exactly_one_visit() {
        let mut tree = Tree::new(Board::new());
        tree.get_node_mut(0).score.wins_white = 1;

        assert_eq!(expand(&mut tree, 0), 20);
        assert_eq!(tree.get_size(), 21);
    }
}

use crate::common::random::Random;
use crate::model::selection_result::SelectionResult;
use crate::model::tree::Tree;
use crate::model::types::TREE_NODE_ROOT_INDEX;
use crate::move_generator::make_move::make_move;

pub fn select(tree: &Tree, random: &mut Random) -> SelectionResult {
    let mut best_node_indices = vec![TREE_NODE_ROOT_INDEX];
    let mut board = tree.get_board().clone();

    loop {
        let random_best_node_index = *random
            .pick_element(&best_node_indices)
            .expect("the list of best nodes must never be empty");

        let parent = tree.get_node(random_best_node_index);
        if random_best_node_index != TREE_NODE_ROOT_INDEX {
            // TODO: rewrite accordingly when last_move is option
            make_move(&mut board, &parent.last_move);
        }
        if parent.child_indices.is_empty() {
            return SelectionResult {
                board,
                node_index: random_best_node_index,
            };
        }

        let mut best_uct = std::u32::MIN;
        best_node_indices = Vec::new();

        for node_index in &parent.child_indices {
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
    use crate::model::types::square_names::*;

    use super::*;

    #[test]
    fn it_selects_and_returns_the_starting_board_when_tree_is_empty() {
        let tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        assert_eq!(
            select(&tree, &mut random),
            SelectionResult {
                board: Board::new(),
                node_index: TREE_NODE_ROOT_INDEX,
            }
        );
    }

    #[test]
    fn it_selects_and_returns_the_resulting_board() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        tree.add_node(Board::new(), Move::from_to(G1, F3), 0);
        tree.add_node(Board::new(), Move::from_to(F7, F6), 1);
        tree.add_node(Board::new(), Move::from_to(E2, E3), 2);
        tree.add_node(Board::new(), Move::from_to(E7, E6), 3);

        assert_eq!(
            select(&tree, &mut random),
            SelectionResult {
                board: Board::from_fen("rnbqkbnr/pppp2pp/4pp2/8/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 3"),
                node_index: 4,
            },
        );
    }

    #[test]
    fn it_selects_the_root_not_in_an_otherwise_empty_tree() {
        let tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);
        assert_eq!(select(&tree, &mut random).node_index, 0);
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

        assert_eq!(select(&tree, &mut random).node_index, 2);
    }

    #[test]
    fn it_selects_unvisited_nodes_first_and_chooses_at_random_among_them() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 1;
        tree.get_node_mut(1).score.wins_white = 1;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree, &mut random).node_index, 4);
        assert_eq!(select(&tree, &mut random).node_index, 4);
        assert_eq!(select(&tree, &mut random).node_index, 2);
        assert_eq!(select(&tree, &mut random).node_index, 4);
        assert_eq!(select(&tree, &mut random).node_index, 4);
        assert_eq!(select(&tree, &mut random).node_index, 4);
        assert_eq!(select(&tree, &mut random).node_index, 2);
    }

    #[test]
    fn it_selects_the_node_with_the_highest_uct() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);
        let mut black_board = Board::new();
        black_board.swap_color();

        tree.add_node(black_board.clone(), Move::from_to(0, 0), 0);
        tree.add_node(black_board.clone(), Move::from_to(0, 0), 0);
        tree.add_node(black_board.clone(), Move::from_to(0, 0), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 2;

        tree.get_node_mut(1).score.wins_black = 1;
        tree.get_node_mut(2).score.wins_white = 1;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree, &mut random).node_index, 2);
    }

    #[test]
    fn it_selects_the_node_with_the_highest_uct_even_if_conclusive() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);
        let mut black_board = Board::new();
        black_board.swap_color();

        tree.add_node(black_board.clone(), Move::from_to(0, 0), 0);
        tree.add_node(black_board.clone(), Move::from_to(0, 0), 0);
        tree.add_node(black_board.clone(), Move::from_to(0, 0), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 2;

        tree.get_node_mut(1).score.wins_black = 1;
        tree.get_node_mut(2).score.wins_white = 1;
        tree.get_node_mut(2).evaluation = BoardEvaluation::WinWhite;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree, &mut random).node_index, 2);
    }

    #[test]
    fn it_selects_node_at_random_if_uct_is_the_same() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);

        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).score.wins_black = 2;

        tree.get_node_mut(1).score.wins_black = 1;
        tree.get_node_mut(2).score.wins_black = 1;
        tree.get_node_mut(3).score.wins_black = 1;

        assert_eq!(select(&tree, &mut random).node_index, 1);
        assert_eq!(select(&tree, &mut random).node_index, 2);
        assert_eq!(select(&tree, &mut random).node_index, 1);
        assert_eq!(select(&tree, &mut random).node_index, 2);
        assert_eq!(select(&tree, &mut random).node_index, 3);
    }
}

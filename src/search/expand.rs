use crate::common::random::Random;
use crate::model::board::Board;
use crate::model::expansion_result::ExpansionResult;
use crate::model::tree::Tree;
use crate::model::types::TreeNodeIndex;
use crate::move_generator::legal_moves::generate_moves;
use crate::move_generator::make_move::make_move;

pub fn expand(
    tree: &mut Tree,
    node_index: TreeNodeIndex,
    board: Board,
    random: &mut Random,
) -> ExpansionResult {
    let mut board = board;
    let node = tree.get_node_mut(node_index);

    if node.evaluation.is_conclusive() || node.is_not_visited() {
        return ExpansionResult { board, node_index };
    }

    debug_assert!(node.child_indices.is_empty());

    let moves = generate_moves(&mut board);
    for m in moves {
        let mut new_board = board.clone();
        make_move(&mut new_board, &m);
        tree.add_node(new_board, m.clone(), node_index);
    }

    let node_index = *random
        .pick_element(&tree.get_node(node_index).child_indices)
        .expect(
            "there must be an expanded node if the parent was inconclusive",
        );

    let last_move = tree.get_node(node_index).last_move;
    make_move(&mut board, &last_move);

    ExpansionResult { board, node_index }
}

#[cfg(test)]
mod test {
    use crate::model::board::Board;
    use crate::model::board_evaluation::BoardEvaluation;
    use crate::model::r#move::Move;
    use crate::model::types::square_names::*;

    use super::*;

    #[test]
    fn it_does_not_expand_a_conclusive_node() {
        let mut tree = Tree::new(Board::new());
        tree.get_node_mut(0).score.wins_white = 1;
        tree.get_node_mut(0).evaluation = BoardEvaluation::WinWhite;
        let mut random = Random::from_seed(111);
        assert_eq!(
            expand(&mut tree, 0, Board::new(), &mut random),
            ExpansionResult {
                board: Board::new(),
                node_index: 0
            },
        );
        assert_eq!(tree.get_size(), 1);
    }

    #[test]
    fn it_does_not_expand_a_node_which_has_no_visits() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);
        assert_eq!(
            expand(&mut tree, 0, Board::new(), &mut random),
            ExpansionResult {
                board: Board::new(),
                node_index: 0
            },
        );
        assert_eq!(tree.get_size(), 1);
    }

    #[test]
    #[should_panic]
    fn it_panics_when_trying_to_expand_a_node_with_children() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);
        tree.add_node(Board::new(), Move::from_to(0, 0), 0);
        tree.get_node_mut(0).score.wins_white = 1;
        expand(&mut tree, 0, Board::new(), &mut random);
    }

    #[test]
    fn it_expands_a_node_with_exactly_one_visit() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);
        tree.get_node_mut(0).score.wins_white = 1;
        expand(&mut tree, 0, Board::new(), &mut random);
        assert_eq!(tree.get_size(), 21);
    }

    #[test]
    fn it_expands_a_node_and_returns_an_updated_board() {
        let mut tree = Tree::new(Board::new());
        let mut random = Random::from_seed(111);

        tree.add_node(Board::new(), Move::from_to(E2, E3), 0);
        tree.add_node(Board::new(), Move::from_to(F7, F6), 1);
        tree.add_node(Board::new(), Move::from_to(D1, H5), 2);
        tree.get_node_mut(3).score.wins_white = 1;

        assert_eq!(
            expand(
                &mut tree,
                3,
                Board::from_fen(
                    "rnbqkbnr/ppppp1pp/5p2/7Q/8/4P3/PPPP1PPP/RNB1KBNR b KQkq - 1 2",
                ),
                &mut random,
            ),
            ExpansionResult {
                board: Board::from_fen("rnbqkbnr/ppppp2p/5pp1/7Q/8/4P3/PPPP1PPP/RNB1KBNR w KQkq - 0 3"),
                node_index: 4,
            }
        );

        assert_eq!(tree.get_size(), 5);
    }
}

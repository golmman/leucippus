use crate::common::random::Random;
use crate::model::board::Board;
use crate::model::tree::Tree;
use crate::model::tree_node_metrics::TreeNodeMetrics;
use crate::view::print_metrics::print_metrics;

use super::backpropagate::backpropagate;
use super::expand::expand;
use super::interpret::get_search_result;
use super::select::select;
use super::simulate::simulate;

pub fn search(board: Board) -> Vec<TreeNodeMetrics> {
    search_iterations(board, std::usize::MAX, true)
}

pub fn search_iterations(
    board: Board,
    max_iterations: usize,
    show_metrics: bool,
) -> Vec<TreeNodeMetrics> {
    let mut tree = Tree::new(board);
    let mut random = Random::from_seed(111);

    for i in 0..max_iterations {
        let node_index = select(&tree, &mut random);
        let node_index = expand(&mut tree, node_index, &mut random);
        let simulation_result = simulate(&tree, node_index, &mut random);
        backpropagate(&mut tree, node_index, simulation_result);

        if show_metrics {
            print_metrics(&tree, i, max_iterations);
        }
    }

    get_search_result(&tree)
}

#[cfg(test)]
mod test {
    use crate::model::board_evaluation::BoardEvaluation;
    use crate::model::r#move::Move;
    use crate::model::types::square_names::*;

    use super::*;

    #[test]
    fn it_finds_the_mate_in_1() {
        let board = Board::from_fen("7k/7p/5N1P/8/8/8/2q5/K7 w - - 0 1");
        let metrics = search_iterations(board, 8, true);
        assert_eq!(metrics[0].last_move, Move::from_to(F6, H7));
        assert_eq!(metrics[0].evaluation, BoardEvaluation::WinWhite);
    }

    #[ignore]
    #[test]
    fn it_finds_the_single_best_move_and_the_two_instant_losing_moves() {
        let board = Board::from_fen(
            "rnbqkbnr/1ppppppp/pB6/8/8/2P2P2/PP1PP1PP/RNB1K1NR b KQkq - 0 1",
        );
        let metrics = search_iterations(board, 20000, true);

        assert_eq!(metrics[0].last_move, Move::from_to(C7, B6));
        assert_eq!(
            metrics[metrics.len() - 2].evaluation,
            BoardEvaluation::WinWhite
        );
        assert_eq!(
            metrics[metrics.len() - 1].evaluation,
            BoardEvaluation::WinWhite
        );
    }
}

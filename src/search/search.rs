use crate::common::random::Random;
use crate::model::args::Args;
use crate::model::board::Board;
use crate::model::tree::Tree;
use crate::model::tree_node_metrics::TreeNodeMetrics;
use crate::view::print_metrics::print_metrics;

use super::backpropagate::backpropagate;
use super::expand::expand;
use super::interpret::get_search_result;
use super::select::select;
use super::simulate::simulate;

pub fn search(args: Args) -> Vec<TreeNodeMetrics> {
    let board = Board::from_fen(&args.fen);
    let mut tree = Tree::new(board);
    let mut random = Random::from_seed(args.seed);

    for i in 1..=args.max_iterations {
        let s = select(&tree, &mut random);
        let e = expand(&mut tree, s.node_index, s.board, &mut random);
        let simulation_result = simulate(&tree, e.node_index, e.board, &mut random);
        backpropagate(&mut tree, e.node_index, simulation_result);

        print_metrics(&tree, i, &args);
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
        let mut args = Args::default();
        args.fen = String::from("7k/7p/5N1P/8/8/8/2q5/K7 w - - 0 1");
        args.max_iterations = 8;
        let metrics = search(args);
        assert_eq!(metrics[0].last_move, Move::from_to(F6, H7));
        assert_eq!(metrics[0].evaluation, BoardEvaluation::WinWhite);
    }

    #[ignore]
    #[test]
    fn it_finds_the_single_best_move_and_the_two_instant_losing_moves() {
        let mut args = Args::default();
        args.fen = String::from(
            "rnbqkbnr/1ppppppp/pB6/8/8/2P2P2/PP1PP1PP/RNB1K1NR b KQkq - 0 1",
        );
        args.max_iterations = 20000;
        args.seed = 111;
        let metrics = search(args);

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

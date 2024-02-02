use std::cmp::Ordering;

use crate::common::random::Random;
use crate::model::board::Board;
use crate::model::board_evaluation::BoardEvaluation;
use crate::model::r#move::Move;
use crate::model::tree::Tree;
use crate::model::tree_node::TreeNode;
use crate::model::tree_node::TreeNodeScore;
use crate::view::print_metrics::print_metrics;

use super::backpropagate::backpropagate;
use super::expand::expand;
use super::select::select;
use super::simulate::simulate;

pub fn search(board: Board) {
    let mut tree = Tree::new(board);

    // mate in 3
    //let mut tree = Tree::new(Board::from_fen(
    //    "rnbqkbnr/1ppppppp/p7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2",
    //));
    // mate in 1
    //let mut tree = Tree::new(Board::from_fen(
    //    "rnbqkbnr/1pppppp1/p6p/4N3/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 3",
    //));

    //let mut tree = Tree::new(Board::from_fen(
    //    "rnbqkbnr/ppppp2p/5p2/6p1/8/4P2N/PPPP1PPP/RNBQKB1R w KQkq - 0 3",
    //));
    //let mut tree = Tree::new(Board::from_fen(
    //    "rnbqk2r/2pppp1p/pp5R/8/2n5/4P3/PPPP1PP1/RNBQKB2 w Qkq - 1 9",
    //));
    //let mut tree = Tree::new(Board::from_fen(
    //    "r4rk1/4n2p/1p2pp2/p1p4P/P3P3/1P2b1P1/8/RN3RK1 w - - 1 19",
    //));
    let mut random = Random::from_seed(111);

    let max = 150000;
    for i in 0..max {
        let node_index = select(&tree, &mut random);
        let node_index = expand(&mut tree, node_index, &mut random);
        let simulation_result = simulate(&tree, node_index, &mut random);
        backpropagate(&mut tree, node_index, simulation_result);

        print_metrics(&tree, i, max);
    }
}

#[cfg(test)]
mod test {
    use super::*;
}

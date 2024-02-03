use crate::common::random::Random;
use crate::model::board::Board;
use crate::model::tree::Tree;
use crate::view::print_metrics::print_metrics;

use super::backpropagate::backpropagate;
use super::expand::expand;
use super::select::select;
use super::simulate::simulate;

pub fn search(board: Board) {
    let mut tree = Tree::new(board);
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
mod test {}

use std::cmp::Ordering;

use crate::common::random::Random;
use crate::model::board::Board;
use crate::model::board_evaluation::BoardEvaluation;
use crate::model::r#move::Move;
use crate::model::tree::Tree;
use crate::model::tree_node::TreeNode;
use crate::model::tree_node::TreeNodeScore;

use super::backpropagate::backpropagate;
use super::expand::expand;
use super::select::select;
use super::simulate::simulate;

pub fn search(board: Board) {
    //let mut tree = Tree::new(board);
    let mut tree = Tree::new(Board::from_fen(
        "rnbqkbnr/1ppppppp/p7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2",
    ));
    let mut random = Random::from_seed(111);

    let max = 200000;
    for i in 0..max {
        let node_index = select(&tree);
        let node_index = expand(&mut tree, node_index);
        let simulation_result = simulate(&tree, node_index, &mut random);
        backpropagate(&mut tree, node_index, simulation_result);

        if i % 100 == 0 {
            println!("{}/{}", i, max);
            show(&tree);
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct TreeNodeInfo {
    pub score: TreeNodeScore,
    pub last_move: Move,
    pub evaluation: BoardEvaluation,
}

impl From<&TreeNode> for TreeNodeInfo {
    fn from(node: &TreeNode) -> Self {
        Self {
            score: node.score.clone(),
            last_move: node.last_move,
            evaluation: node.evaluation,
        }
    }
}

impl PartialOrd for TreeNodeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.evaluation == BoardEvaluation::WinWhite
            && other.evaluation == BoardEvaluation::WinWhite
        {
            return Some(Ordering::Equal);
        }
        if self.evaluation != BoardEvaluation::WinWhite
            && other.evaluation == BoardEvaluation::WinWhite
        {
            return Some(Ordering::Greater);
        }
        if self.evaluation == BoardEvaluation::WinWhite
            && other.evaluation != BoardEvaluation::WinWhite
        {
            return Some(Ordering::Less);
        }

        Some(other.score.wins_white.cmp(&self.score.wins_white))
        //Some(other.score.wins_black.cmp(&self.score.wins_black))
    }
}

impl Ord for TreeNodeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.evaluation == BoardEvaluation::WinWhite
            && other.evaluation == BoardEvaluation::WinWhite
        {
            return Ordering::Equal;
        }
        if self.evaluation != BoardEvaluation::WinWhite
            && other.evaluation == BoardEvaluation::WinWhite
        {
            return Ordering::Greater;
        }
        if self.evaluation == BoardEvaluation::WinWhite
            && other.evaluation != BoardEvaluation::WinWhite
        {
            return Ordering::Less;
        }

        other.score.wins_white.cmp(&self.score.wins_white)
        //other.score.wins_black.cmp(&self.score.wins_black)
    }
}

fn show(tree: &Tree) {
    let root_node = tree.get_root();

    let mut infos: Vec<TreeNodeInfo> = root_node
        .child_indices
        .iter()
        .map(|c| TreeNodeInfo::from(tree.get_node(*c)))
        .collect();

    infos.sort();

    println!("tree size: {}", tree.get_size());

    for info in &infos {
        print!("|{:02},{:02}", info.last_move.from, info.last_move.to);
    }
    println!("|");

    for info in &infos {
        match info.evaluation {
            BoardEvaluation::Draw => print!("|0    "),
            BoardEvaluation::Inconclusive => print!("|?    "),
            BoardEvaluation::WinBlack => print!("|Black"),
            BoardEvaluation::WinWhite => print!("|White"),
        }
    }
    println!("|");

    for info in &infos {
        //print!(
        //    "|{:03}{:03}{:03}",
        //    info.score.draws, info.score.wins_black, info.score.wins_white
        //);
        print!("|{:05}", info.score.wins_white);
        //print!("|{:09}", info.score.wins_black);
    }
    println!("|");
}

#[cfg(test)]
mod test {
    use super::*;
}

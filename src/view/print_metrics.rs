use std::cmp::Ordering;

use crate::model::board_evaluation::BoardEvaluation;
use crate::model::r#move::Move;
use crate::model::tree::Tree;
use crate::model::tree_node::TreeNode;
use crate::model::tree_node::TreeNodeScore;

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

pub fn print_metrics(tree: &Tree, iteration: i32, max_iteration: i32) {
    if iteration % 100 != 0 {
        return;
    }
    println!("{}/{}", iteration, max_iteration);

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
        let total = (info.score.draws
            + info.score.wins_black
            + info.score.wins_white) as f32;
        print!(
            "|{:>5.1}",
            100.0
                * (info.score.wins_white as i32 - info.score.wins_black as i32)
                    as f32
                / total
        );
        //print!("|{:09}", info.score.wins_black);
    }
    println!("|");

    for info in &infos {
        //print!(
        //    "|{:03}{:03}{:03}",
        //    info.score.draws, info.score.wins_black, info.score.wins_white
        //);
        let total =
            info.score.draws + info.score.wins_black + info.score.wins_white;
        print!("|{:05}", total);
        //print!("|{:09}", info.score.wins_black);
    }
    println!("|");
}

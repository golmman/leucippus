use std::cmp::Ordering;

use crate::model::board_evaluation::BoardEvaluation;
use crate::model::color::Color;
use crate::model::r#move::Move;
use crate::model::tree::Tree;
use crate::model::tree_node::TreeNode;
use crate::model::tree_node::TreeNodeScore;
use crate::model::tree_node_metrics::compare_black;
use crate::model::tree_node_metrics::compare_white;
use crate::model::tree_node_metrics::TreeNodeMetrics;
use crate::model::types::square_names::SQUARE_NAMES;

pub fn print_metrics(tree: &Tree, iteration: i32, max_iteration: i32) {
    if iteration % 50 != 0 {
        return;
    }
    println!("{}/{}", iteration, max_iteration);
    println!("tree size: {}", tree.get_size());

    let root_node = tree.get_root();

    let mut infos: Vec<TreeNodeMetrics> = root_node
        .child_indices
        .iter()
        .map(|c| TreeNodeMetrics::from(tree.get_node(*c)))
        .collect();

    if tree.get_node(0).board.our_color == Color::Black {
        infos.sort_by(compare_black);
    } else {
        infos.sort_by(compare_white);
    }

    for info in &infos {
        print!(
            "|{:02},{:02}",
            SQUARE_NAMES[info.last_move.from as usize],
            SQUARE_NAMES[info.last_move.to as usize]
        );
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

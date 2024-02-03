use crate::model::board_evaluation::BoardEvaluation;
use crate::model::tree::Tree;
use crate::model::types::square_names::SQUARE_NAMES;
use crate::search::interpret::get_search_result;

pub fn print_metrics(tree: &Tree, iteration: usize, max_iteration: usize) {
    if iteration % 50 != 0 {
        return;
    }
    println!("{}/{}", iteration, max_iteration);
    println!("tree size: {}", tree.get_size());

    let infos = get_search_result(&tree);

    print!("Move       ");
    for info in &infos {
        print!(
            "|{},{}",
            SQUARE_NAMES[info.last_move.from as usize],
            SQUARE_NAMES[info.last_move.to as usize]
        );
    }
    println!("|");

    print!("Evaluation ");
    for info in &infos {
        match info.evaluation {
            BoardEvaluation::Draw => print!("|0    "),
            BoardEvaluation::Inconclusive => print!("|?    "),
            BoardEvaluation::WinBlack => print!("|Black"),
            BoardEvaluation::WinWhite => print!("|White"),
        }
    }
    println!("|");

    print!("Score      ");
    for info in &infos {
        let total = (info.score.draws
            + info.score.wins_black
            + info.score.wins_white) as f32;
        let score = info.score.wins_white as i32 - info.score.wins_black as i32;
        print!("|{:>5.1}", 100.0 * score as f32 / total);
    }
    println!("|");

    print!("Simulations");
    for info in &infos {
        let total =
            info.score.draws + info.score.wins_black + info.score.wins_white;
        print!("|{:05}", total);
    }
    println!("|");
}

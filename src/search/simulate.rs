use crate::common::random::Random;
use crate::evaluation::evaluate_board::evaluate_board;
use crate::model::board_evaluation::BoardEvaluation;
use crate::model::simulation_result::SimulationResult;
use crate::model::tree::Tree;
use crate::model::types::TreeNodeIndex;
use crate::move_generator::legal_moves::generate_moves;
use crate::move_generator::make_move::make_move;

pub fn simulate(
    tree: &Tree,
    node_index: TreeNodeIndex,
    random: &mut Random,
) -> SimulationResult {
    // TODO: should be easy to parallelize this
    // TODO: threefold repetition check

    let node = tree.get_node(node_index);
    debug_assert!(node.is_not_visited());

    let mut board = node.board.clone();
    let mut board_hashes = get_principal_variation_hashes(tree, node_index);
    let mut last_board_hash = node.board_hash;
    let mut depth = 0;

    loop {
        debug_assert!(depth < 1000);

        if has_three_duplicates(&board_hashes, last_board_hash) {
            board.draw_by_repetition = true;
        }

        let evaluation = evaluate_board(&mut board);
        if evaluation != BoardEvaluation::Inconclusive {
            return SimulationResult { depth, evaluation };
        }

        // TODO: why calculate this twice, add it to the tree?
        let moves = generate_moves(&mut board);
        let random_move = moves[random.next() as usize % moves.len()];
        make_move(&mut board, &random_move);
        last_board_hash = board.get_hash();
        board_hashes.push(last_board_hash);
        depth += 1;
    }
}

fn get_principal_variation_hashes(
    tree: &Tree,
    node_index: TreeNodeIndex,
) -> Vec<u64> {
    let mut index = node_index;
    let mut hashes = Vec::new();

    loop {
        let node = tree.get_node(index);
        hashes.push(node.board_hash);

        let Some(parent) = node.parent_index else {
            return hashes;
        };
        index = parent;
    }
}

fn has_three_duplicates(list: &Vec<u64>, check_value: u64) -> bool {
    let mut strikes = 0;
    for item in list {
        if *item == check_value {
            strikes += 1;
        }

        if strikes >= 3 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::model::board::Board;

    use super::*;

    #[test]
    fn it_gets_all_principal_variation_hashes() {
        let mut tree = Tree::new(Board::new());
        tree.add_node(Board::new(), 0);
        tree.add_node(Board::new(), 1);
        tree.add_node(Board::new(), 2);

        tree.get_node_mut(0).board_hash = 2;
        tree.get_node_mut(1).board_hash = 3;
        tree.get_node_mut(2).board_hash = 5;
        tree.get_node_mut(3).board_hash = 7;

        let hashes = get_principal_variation_hashes(&tree, 3);

        assert_eq!(hashes.len(), 4);
        assert!(hashes.contains(&2));
        assert!(hashes.contains(&3));
        assert!(hashes.contains(&5));
        assert!(hashes.contains(&7));
    }

    #[test]
    fn it_finds_three_duplicates_in_a_list() {
        let list = vec![1, 2, 3, 5, 65, 3, 35, 5, 3, 8];
        assert!(has_three_duplicates(&list, 3));
    }

    #[test]
    fn it_does_not_find_three_duplicates_in_an_empty_list() {
        let list = vec![];
        assert!(!has_three_duplicates(&list, 5));
    }

    #[test]
    fn it_does_not_find_three_duplicates_in_a_list() {
        let list = vec![1, 2, 3, 5, 65, 3, 35, 5, 3, 8];
        assert!(!has_three_duplicates(&list, 5));
    }

    #[test]
    fn it_simulates_moves_from_the_starting_position() {
        let tree = Tree::new(Board::new());
        let mut random = Random::from_seed(0);

        let result = simulate(&tree, 0, &mut random);

        assert_eq!(result.depth, 12);
        assert_eq!(result.evaluation, BoardEvaluation::WinBlack);

        // TODO: remove
        //let mut random = Random::from_seed(999);
        //let mut white_wins = 0;
        //let mut black_wins = 0;
        //let mut draws = 0;
        //for i in 0..1000 {
        //    let result = simulate(&tree, 0, &mut random);
        //    match result.evaluation {
        //        BoardEvaluation::Draw => draws += 1,
        //        BoardEvaluation::WinBlack => black_wins += 1,
        //        BoardEvaluation::WinWhite => white_wins += 1,
        //        _ => panic!(),
        //    }
        //}
        //println!("white: {}, black: {}, draws: {}", white_wins, black_wins, draws);
    }

    #[test]
    fn it_simulates_moves_for_a_board_with_forced_stalemate() {
        let board = Board::from_fen("kb6/p1p5/P1P4p/8/7p/7P/8/2K5 w - - 0 1");
        let tree = Tree::new(board);
        let mut random = Random::from_seed(0);

        let result = simulate(&tree, 0, &mut random);

        assert_eq!(result.depth, 3);
        assert_eq!(result.evaluation, BoardEvaluation::Draw);
    }

    #[test]
    fn it_simulates_moves_for_a_board_with_forced_checkmate() {
        let board = Board::from_fen(
            "k4BRR/p1p1q1PP/P1P4P/7p/8/p1p5/P1P2r2/KB6 w - - 0 1",
        );
        let tree = Tree::new(board);
        let mut random = Random::from_seed(0);

        let result = simulate(&tree, 0, &mut random);

        assert_eq!(result.depth, 3);
        assert_eq!(result.evaluation, BoardEvaluation::WinWhite);
    }

    #[test]
    fn it_simulates_moves_for_a_board_with_draw_because_of_insufficient_material(
    ) {
        let board = Board::from_fen("7k/6p1/5NB1/8/2n4B/1nn5/2P5/K7 w - - 0 1");
        let tree = Tree::new(board);
        let mut random = Random::from_seed(0);

        let result = simulate(&tree, 0, &mut random);

        assert_eq!(result.depth, 2);
        assert_eq!(result.evaluation, BoardEvaluation::Draw);
    }

    #[test]
    fn it_simulates_moves_for_a_board_with_draw_because_of_50_moves_rule() {
        let board = Board::from_fen("8/8/3b1K2/8/4B3/2k5/8/8 w - - 95 200");
        let tree = Tree::new(board);
        let mut random = Random::from_seed(0);

        let result = simulate(&tree, 0, &mut random);

        assert_eq!(result.depth, 5);
        assert_eq!(result.evaluation, BoardEvaluation::Draw);
    }

    #[test]
    fn it_simulates_moves_for_a_board_with_draw_because_of_threefold_repetition(
    ) {
        let board = Board::from_fen(
            "1kb5/1p1p4/rP1P4/1P6/8/1p1p4/1P1P4/1KB5 w - - 0 1",
        );
        let tree = Tree::new(board);
        let mut random = Random::from_seed(0);

        let result = simulate(&tree, 0, &mut random);

        assert_eq!(result.depth, 9);
        assert_eq!(result.evaluation, BoardEvaluation::Draw);
    }
}

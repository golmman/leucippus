use crate::model::board::Board;
use crate::model::types::SQUARES_TOUCH;
use crate::move_generator::make_move::make_null_move;

use super::legal_moves;

pub fn is_check(board: &mut Board) -> bool {
    let our_king_index = board.pieces.active_kings[0];

    make_null_move(board);
    let their_king_index = board.pieces.active_kings[0];

    if SQUARES_TOUCH[our_king_index as usize][their_king_index as usize] == 1 {
        make_null_move(board);
        return false;
    }

    let moves = legal_moves::generate_pseudo_legal(board);
    for m in moves {
        if m.to == our_king_index {
            make_null_move(board);
            return true;
        }
    }
    make_null_move(board);

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn white_is_not_in_check_in_the_starting_position() {
        let mut board = Board::new();
        assert!(!is_check(&mut board));
    }

    #[test]
    fn black_is_in_check_by_a_single_knight() {
        let fen = "4k3/8/5N2/8/8/8/8/1K6 b - - 0 1";
        let mut board = Board::from_fen(fen);
        assert!(is_check(&mut board));
    }

    #[test]
    fn white_is_in_check_by_a_single_knight() {
        let fen = "4k3/8/8/8/8/2n5/8/1K6 w - - 0 1";
        let mut board = Board::from_fen(fen);
        assert!(is_check(&mut board));
    }

    #[test]
    fn black_not_in_check_because_kings_touch() {
        let fen = "8/8/3Kk3/8/2n2N2/8/8/8 b - - 0 1";
        let mut board = Board::from_fen(fen);
        assert!(!is_check(&mut board));
    }

    #[test]
    fn white_not_in_check_because_kings_touch() {
        let fen = "8/8/3Kk3/8/2n2N2/8/8/8 w - - 0 1";
        let mut board = Board::from_fen(fen);
        assert!(!is_check(&mut board));
    }

    #[test]
    fn black_in_check_in_a_complex_position() {
        let fen =
            "2bqk2r/1p4pp/4pp1n/1B1p4/1b6/4P3/PPPP1PPP/R1BQK2R b KQk - 2 8";
        let mut board = Board::from_fen(fen);
        assert!(is_check(&mut board));
    }

    #[test]
    fn it_proves_the_board_is_not_changed_after_check_evaluation() {
        let fen =
            "2bqk2r/1p4pp/4pp1n/1B1p4/1b6/4P3/PPPP1PPP/R1BQK2R b KQk - 2 8";
        let mut board = Board::from_fen(fen);
        is_check(&mut board);
        assert_eq!(board, Board::from_fen(fen));
    }
}

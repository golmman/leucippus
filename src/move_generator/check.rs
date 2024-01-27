use crate::model::board::Board;
use crate::model::r#move::Move;
use crate::model::r#move::MoveSpecial;
use crate::model::types::SQUARES_TOUCH;
use crate::move_generator::make_move::make_null_move;

use super::legal_moves;

pub fn is_check(board: &mut Board) -> bool {
    let our_king_index = board.pieces.active_kings[0];

    make_null_move(board);
    let Some(their_king_index) = board.pieces.active_kings.get(0) else {
        // king might have exploded already
        return false;
    };

    if SQUARES_TOUCH[our_king_index as usize][*their_king_index as usize] == 1 {
        make_null_move(board);
        return false;
    }

    let moves = legal_moves::generate_moves_pseudo_legal_without_kings(board);
    for m in moves {
        if m.to == our_king_index {
            make_null_move(board);
            return true;
        }
    }

    make_null_move(board);
    false
}

pub fn is_legal_castling(board: &mut Board, m: &Move) -> bool {
    let castling_checks_squares: [u8; 3] = match m.special {
        Some(MoveSpecial::CastleLongBlack) => [58, 59, 60],
        Some(MoveSpecial::CastleShortBlack) => [60, 61, 62],
        Some(MoveSpecial::CastleLongWhite) => [2, 3, 4],
        Some(MoveSpecial::CastleShortWhite) => [4, 5, 6],
        _ => return true,
    };

    make_null_move(board);
    let their_king_index = board.pieces.active_kings[0];

    let moves = legal_moves::generate_moves_pseudo_legal_without_kings(board);
    for c in castling_checks_squares {
        if SQUARES_TOUCH[their_king_index as usize][c as usize] == 1 {
            // their king touches a castling square so it can't be attacked
            continue;
        }
        for m in &moves {
            if m.to == c {
                // a relevant castling square is attacked
                make_null_move(board);
                return false;
            }
        }
    }

    make_null_move(board);
    true
}

#[cfg(test)]
mod test {
    use super::*;

    mod is_check {
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

    mod is_castling_allowed {
        use super::*;
        #[test]
        fn it_allows_black_castling_short() {
            let mut board = Board::from_fen(
                "r3k2r/8/1K6/2R1b3/4p1p1/BPP2p2/P2P1PP1/8 b kq - 29 36",
            );
            assert!(is_legal_castling(&mut board, &Move::castle_short_black()));
        }

        #[test]
        fn it_disallows_black_castling_long() {
            let mut board = Board::from_fen(
                "r3k2r/8/1K6/2R1b3/4p1p1/BPP2p2/P2P1PP1/8 b kq - 29 36",
            );
            assert!(!is_legal_castling(&mut board, &Move::castle_long_black()));
        }

        #[test]
        fn it_allows_black_castling_long_because_of_their_king_touching() {
            let mut board = Board::from_fen(
                "r3k2r/1K6/8/2R5/3bp1p1/BPP2p2/P2P1PP1/8 b kq - 27 35",
            );
            assert!(is_legal_castling(&mut board, &Move::castle_long_black()));
        }
    }
}

use crate::model::board::Board;
use crate::model::r#move::Move;

use super::bishop_moves;
use super::check::is_check;
use super::check::is_legal_castling;
use super::king_moves;
use super::knight_moves;
use super::make_move::make_move;
use super::make_move::move_piece;
use super::pawn_moves;
use super::queen_moves;
use super::rook_moves;

pub fn generate_pseudo_legal_without_kings(board: &mut Board) -> Vec<Move> {
    let mut moves = Vec::new();

    moves.append(&mut bishop_moves::generate(board));
    moves.append(&mut knight_moves::generate(board));
    moves.append(&mut pawn_moves::generate(board));
    moves.append(&mut queen_moves::generate(board));
    moves.append(&mut rook_moves::generate(board));

    moves
}

pub fn generate_pseudo_legal(board: &mut Board) -> Vec<Move> {
    let mut moves = Vec::new();

    moves.append(&mut bishop_moves::generate(board));
    moves.append(&mut king_moves::generate(board));
    moves.append(&mut knight_moves::generate(board));
    moves.append(&mut pawn_moves::generate(board));
    moves.append(&mut queen_moves::generate(board));
    moves.append(&mut rook_moves::generate(board));

    moves
}

pub fn generate(board: &mut Board) -> Vec<Move> {
    let moves = generate_pseudo_legal(board);

    // filter moves which leave the king in check
    //    even if in check allow explosion of their king

    // filter illegal castlings

    // filter direct captures of kings, saves some indirect calculations

    moves
        .into_iter()
        .filter(|m| {
            true && is_no_king_capture(board, m)
                && is_no_illegal_castle(board, m)
                && does_not_leave_king_in_check(board, m)
        })
        .collect()
}

fn is_no_king_capture(board: &Board, m: &Move) -> bool {
    !board.has_king_at(m.to)
}

fn is_no_illegal_castle(board: &mut Board, m: &Move) -> bool {
    if !m.is_castle() {
        return true;
    }

    is_legal_castling(board, m)
}

fn does_not_leave_king_in_check(board: &Board, m: &Move) -> bool {
    let mut board_clone = board.clone();

    move_piece(&mut board_clone, m);

    if board_clone.pieces.active_kings.len() == 0 {
        return true;
    }

    !is_check(&mut board_clone)
}

#[cfg(test)]
mod test {
    use crate::model::types::square_names::*;

    use super::*;

    #[test]
    fn it_generates_all_legal_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = generate(&mut board);
        assert_eq!(moves.len(), 20);
        assert_eq!(
            moves,
            vec![
                Move::from_to(1, 16),
                Move::from_to(1, 18),
                Move::from_to(6, 21),
                Move::from_to(6, 23),
                Move::from_to(8, 16),
                Move::from_to(8, 24),
                Move::from_to(9, 17),
                Move::from_to(9, 25),
                Move::from_to(10, 18),
                Move::from_to(10, 26),
                Move::from_to(11, 19),
                Move::from_to(11, 27),
                Move::from_to(12, 20),
                Move::from_to(12, 28),
                Move::from_to(13, 21),
                Move::from_to(13, 29),
                Move::from_to(14, 22),
                Move::from_to(14, 30),
                Move::from_to(15, 23),
                Move::from_to(15, 31),
            ]
        );
    }

    #[test]
    fn it_generates_all_legal_moves_from_a_position_with_complex_castling() {
        let fen = "r3k2r/1K6/8/2R5/3bp1p1/BPP2p2/P2P1PP1/8 b kq - 27 35";
        let mut board = Board::from_fen(fen);
        let moves = generate(&mut board);

        assert_eq!(moves.len(), 34);

        // 8 rook moves
        assert!(moves.contains(&Move::from_to(A8, A7)));
        assert!(moves.contains(&Move::from_to(A8, A6)));
        assert!(moves.contains(&Move::from_to(A8, A5)));
        assert!(moves.contains(&Move::from_to(A8, A4)));
        assert!(moves.contains(&Move::from_to(A8, A3)));
        assert!(moves.contains(&Move::from_to(A8, B8)));
        assert!(moves.contains(&Move::from_to(A8, C8)));
        assert!(moves.contains(&Move::from_to(A8, D8)));

        // 9 rook moves
        assert!(moves.contains(&Move::from_to(H8, G8)));
        assert!(moves.contains(&Move::from_to(H8, F8)));
        assert!(moves.contains(&Move::from_to(H8, H7)));
        assert!(moves.contains(&Move::from_to(H8, H6)));
        assert!(moves.contains(&Move::from_to(H8, H5)));
        assert!(moves.contains(&Move::from_to(H8, H4)));
        assert!(moves.contains(&Move::from_to(H8, H3)));
        assert!(moves.contains(&Move::from_to(H8, H2)));
        assert!(moves.contains(&Move::from_to(H8, H1)));

        // 7 king moves
        assert!(moves.contains(&Move::from_to(E8, D8)));
        assert!(moves.contains(&Move::from_to(E8, F8)));
        assert!(moves.contains(&Move::from_to(E8, D7)));
        assert!(moves.contains(&Move::from_to(E8, E7)));
        assert!(moves.contains(&Move::from_to(E8, F7)));
        assert!(moves.contains(&Move::castle_long_black()));
        assert!(moves.contains(&Move::castle_short_black()));

        // 7 bishop moves
        assert!(moves.contains(&Move::from_to(D4, C5)));
        assert!(moves.contains(&Move::from_to(D4, C3)));
        assert!(moves.contains(&Move::from_to(D4, E3)));
        assert!(moves.contains(&Move::from_to(D4, F2)));
        assert!(moves.contains(&Move::from_to(D4, E5)));
        assert!(moves.contains(&Move::from_to(D4, F6)));
        assert!(moves.contains(&Move::from_to(D4, G7)));

        // 3 pawn moves
        assert!(moves.contains(&Move::from_to(E4, E3)));
        assert!(moves.contains(&Move::from_to(F3, G2)));
        assert!(moves.contains(&Move::from_to(G4, G3)));
    }

    mod simple_positions {
        use super::*;

        #[test]
        fn it_generates_all_legal_moves_for_black_where_one_puts_the_their_king_in_check(
        ) {
            let fen = "2BK3n/8/8/8/8/8/8/7k b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(H8, F7)));
            assert!(moves.contains(&Move::from_to(H8, G6)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, G2)));
            assert!(moves.contains(&Move::from_to(H1, H2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_one_puts_the_their_king_in_check(
        ) {
            let fen = "2bk3N/8/8/8/8/8/8/7K w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(H8, F7)));
            assert!(moves.contains(&Move::from_to(H8, G6)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, G2)));
            assert!(moves.contains(&Move::from_to(H1, H2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_where_our_knight_is_pinned() {
            let fen = "2BK3R/8/8/8/8/7n/8/7k b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            assert_eq!(moves.len(), 3);
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, G2)));
            assert!(moves.contains(&Move::from_to(H1, H2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_our_knight_is_pinned() {
            let fen = "2bk3r/8/8/8/8/7N/8/7K w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            assert_eq!(moves.len(), 3);
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, G2)));
            assert!(moves.contains(&Move::from_to(H1, H2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_where_our_queen_is_pinned_but_may_explode_their_king(
        ) {
            let fen = "2BK3R/8/8/8/8/7q/8/7k b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            assert_eq!(moves.len(), 10);
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, G2)));
            assert!(moves.contains(&Move::from_to(H1, H2)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
            assert!(moves.contains(&Move::from_to(H3, H4)));
            assert!(moves.contains(&Move::from_to(H3, H5)));
            assert!(moves.contains(&Move::from_to(H3, H6)));
            assert!(moves.contains(&Move::from_to(H3, H7)));
            assert!(moves.contains(&Move::from_to(H3, H8)));
            assert!(moves.contains(&Move::from_to(H3, C8)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_our_queen_is_pinned_but_may_explode_their_king(
        ) {
            let fen = "2bk3r/8/8/8/8/7Q/8/7K w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            assert_eq!(moves.len(), 10);
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, G2)));
            assert!(moves.contains(&Move::from_to(H1, H2)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
            assert!(moves.contains(&Move::from_to(H3, H4)));
            assert!(moves.contains(&Move::from_to(H3, H5)));
            assert!(moves.contains(&Move::from_to(H3, H6)));
            assert!(moves.contains(&Move::from_to(H3, H7)));
            assert!(moves.contains(&Move::from_to(H3, H8)));
            assert!(moves.contains(&Move::from_to(H3, C8)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_where_our_king_has_to_move_out_of_check(
        ) {
            let fen = "2BK4/8/8/P7/p7/7k/8/8 b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            println!("{moves:?}");

            assert_eq!(moves.len(), 4);
            assert!(moves.contains(&Move::from_to(H3, H4)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
            assert!(moves.contains(&Move::from_to(H3, G3)));
            assert!(moves.contains(&Move::from_to(H3, G2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_our_king_has_to_move_out_of_check(
        ) {
            let fen = "2bk4/8/8/p7/P7/7K/8/8 w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            println!("{moves:?}");

            assert_eq!(moves.len(), 4);
            assert!(moves.contains(&Move::from_to(H3, H4)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
            assert!(moves.contains(&Move::from_to(H3, G3)));
            assert!(moves.contains(&Move::from_to(H3, G2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_where_our_king_has_to_move_out_of_check_or_explode_their_king(
        ) {
            let fen = "2BK4/8/8/P7/p7/7k/8/2r5 b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            println!("{moves:?}");

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(H3, H4)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
            assert!(moves.contains(&Move::from_to(H3, G3)));
            assert!(moves.contains(&Move::from_to(H3, G2)));
            assert!(moves.contains(&Move::from_to(C1, C8)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_our_king_has_to_move_out_of_check_or_explode_their_king(
        ) {
            let fen = "2bk4/8/8/p7/P7/7K/8/2R5 w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate(&mut board);

            println!("{moves:?}");

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(H3, H4)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
            assert!(moves.contains(&Move::from_to(H3, G3)));
            assert!(moves.contains(&Move::from_to(H3, G2)));
            assert!(moves.contains(&Move::from_to(C1, C8)));
        }
    }
}

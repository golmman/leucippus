use crate::common::random::Random;
use crate::model::board::Board;
use crate::model::r#move::Move;

use super::bishop_moves;
use super::check::is_check;
use super::check::is_legal_castling;
use super::king_moves;
use super::knight_moves;
use super::make_move::move_piece;
use super::pawn_moves;
use super::queen_moves;
use super::rook_moves;

// TODO: provide function which stops move generation for the first legal move

pub fn generate_moves_pseudo_legal_without_kings(
    board: &mut Board,
) -> Vec<Move> {
    let mut moves = Vec::with_capacity(40);

    moves.append(&mut bishop_moves::generate(board));
    moves.append(&mut knight_moves::generate(board));
    moves.append(&mut pawn_moves::generate(board));
    moves.append(&mut queen_moves::generate(board));
    moves.append(&mut rook_moves::generate(board));

    moves
}

pub fn generate_moves_pseudo_legal(board: &mut Board) -> Vec<Move> {
    let mut moves = Vec::with_capacity(40);

    moves.append(&mut bishop_moves::generate(board));
    moves.append(&mut king_moves::generate(board));
    moves.append(&mut knight_moves::generate(board));
    moves.append(&mut pawn_moves::generate(board));
    moves.append(&mut queen_moves::generate(board));
    moves.append(&mut rook_moves::generate(board));

    moves
}

pub fn generate_move(board: &mut Board, random: &mut Random) -> Option<Move> {
    let mut moves = generate_moves_pseudo_legal(board);
    random.shuffle(&mut moves);

    for i in 0..moves.len() {
        let m = moves[i];

        if is_king_capture(board, &m) {
            continue;
        }

        if is_illegal_castle(board, &m) {
            continue;
        }

        if leaves_king_in_check(board, &m) {
            continue;
        }

        return Some(m);
    }

    None
}

pub fn generate_moves(board: &mut Board) -> Vec<Move> {
    let moves = generate_moves_pseudo_legal(board);

    moves
        .into_iter()
        .filter(|m| {
            true && !is_king_capture(board, m)
                && !is_illegal_castle(board, m)
                && !leaves_king_in_check(board, m)
        })
        .collect()
}

fn is_king_capture(board: &Board, m: &Move) -> bool {
    board.has_king_at(m.to)
}

fn is_illegal_castle(board: &mut Board, m: &Move) -> bool {
    if !m.is_castle() {
        return false;
    }

    !is_legal_castling(board, m)
}

fn leaves_king_in_check(board: &Board, m: &Move) -> bool {
    let mut board_clone = board.clone();

    move_piece(&mut board_clone, m);

    if board_clone.pieces.our_kings.is_empty() {
        // our king is no more, so all moves are illegal or "leave the king in check"
        return true;
    }

    is_check(&board_clone)
}

#[cfg(test)]
mod test {
    use crate::model::types::square_names::*;

    use super::*;

    #[test]
    fn it_generates_all_legal_moves_from_the_starting_position() {
        let mut board = Board::new();
        let moves = generate_moves(&mut board);

        assert_eq!(moves.len(), 20);

        assert!(moves.contains(&Move::from_to(1, 16)));
        assert!(moves.contains(&Move::from_to(1, 18)));
        assert!(moves.contains(&Move::from_to(6, 21)));
        assert!(moves.contains(&Move::from_to(6, 23)));
        assert!(moves.contains(&Move::from_to(8, 16)));
        assert!(moves.contains(&Move::from_to(8, 24)));
        assert!(moves.contains(&Move::from_to(9, 17)));
        assert!(moves.contains(&Move::from_to(9, 25)));
        assert!(moves.contains(&Move::from_to(10, 18)));
        assert!(moves.contains(&Move::from_to(10, 26)));
        assert!(moves.contains(&Move::from_to(11, 19)));
        assert!(moves.contains(&Move::from_to(11, 27)));
        assert!(moves.contains(&Move::from_to(12, 20)));
        assert!(moves.contains(&Move::from_to(12, 28)));
        assert!(moves.contains(&Move::from_to(13, 21)));
        assert!(moves.contains(&Move::from_to(13, 29)));
        assert!(moves.contains(&Move::from_to(14, 22)));
        assert!(moves.contains(&Move::from_to(14, 30)));
        assert!(moves.contains(&Move::from_to(15, 23)));
        assert!(moves.contains(&Move::from_to(15, 31)));
    }

    #[test]
    fn it_generates_all_legal_moves_from_a_position_with_complex_castling() {
        let fen = "r3k2r/1K6/8/2R5/3bp1p1/BPP2p2/P2P1PP1/8 b kq - 27 35";
        let mut board = Board::from_fen(fen);
        let moves = generate_moves(&mut board);

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

    mod no_moves {
        use super::*;

        #[test]
        fn it_generates_no_legal_moves_in_a_position_where_the_king_has_exploded(
        ) {
            let fen = "3K3R/8/8/8/8/8/8/3n4 b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 0);
        }

        #[test]
        fn it_generates_no_legal_moves_in_a_stalemate_position() {
            let fen = "3K3R/8/1B6/8/5N2/8/7n/7k b - - 1 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 0);
        }

        #[test]
        fn it_generates_no_legal_moves_in_a_checkmate_position() {
            let fen = "3K3R/8/1BB5/8/5N2/8/7n/7k b - - 2 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 0);
        }

        #[test]
        fn it_generates_no_random_move_in_a_position_where_the_king_has_exploded(
        ) {
            let fen = "3K3R/8/8/8/8/8/8/3n4 b - - 0 1";
            let mut board = Board::from_fen(fen);
            let mut random = Random::from_seed(777);
            let m = generate_move(&mut board, &mut random);

            assert!(m.is_none());
        }

        #[test]
        fn it_generates_no_random_move_in_a_stalemate_position() {
            let fen = "3K3R/8/1B6/8/5N2/8/7n/7k b - - 1 1";
            let mut board = Board::from_fen(fen);
            let mut random = Random::from_seed(777);
            let m = generate_move(&mut board, &mut random);

            assert!(m.is_none());
        }

        #[test]
        fn it_generates_no_random_move_in_a_checkmate_position() {
            let fen = "3K3R/8/1BB5/8/5N2/8/7n/7k b - - 2 1";
            let mut board = Board::from_fen(fen);
            let mut random = Random::from_seed(777);
            let m = generate_move(&mut board, &mut random);

            assert!(m.is_none());
        }
    }

    mod simple_positions {
        use super::*;

        #[test]
        fn it_generates_all_legal_moves_for_black_where_one_puts_the_their_king_in_check(
        ) {
            let fen = "2BK3n/8/8/8/8/8/8/7k b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 3);
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, G2)));
            assert!(moves.contains(&Move::from_to(H1, H2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_where_our_knight_is_pinned_with_random_single_move_generation(
        ) {
            let fen = "2BK3R/8/8/8/8/7n/8/7k b - - 0 1";
            let mut board = Board::from_fen(fen);
            let mut random = Random::from_seed(777);
            let move0 = Some(Move::from_to(H1, G1));
            let move1 = Some(Move::from_to(H1, G2));
            let move2 = Some(Move::from_to(H1, H2));

            assert_eq!(generate_move(&mut board, &mut random), move0);
            assert_eq!(generate_move(&mut board, &mut random), move1);
            assert_eq!(generate_move(&mut board, &mut random), move1);
            assert_eq!(generate_move(&mut board, &mut random), move0);
            assert_eq!(generate_move(&mut board, &mut random), move0);
            assert_eq!(generate_move(&mut board, &mut random), move1);
            assert_eq!(generate_move(&mut board, &mut random), move2);
            assert_eq!(generate_move(&mut board, &mut random), move2);
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_our_knight_is_pinned() {
            let fen = "2bk3r/8/8/8/8/7N/8/7K w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

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
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(H3, H4)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
            assert!(moves.contains(&Move::from_to(H3, G3)));
            assert!(moves.contains(&Move::from_to(H3, G2)));
            assert!(moves.contains(&Move::from_to(C1, C8)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_a_pawn_explosion_whould_leave_our_king_in_check(
        ) {
            let fen = "2bk4/8/8/p4n2/P3pp2/4P2K/8/8 w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 3);
            assert!(moves.contains(&Move::from_to(H3, G4)));
            assert!(moves.contains(&Move::from_to(H3, G2)));
            assert!(moves.contains(&Move::from_to(H3, H2)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_where_a_capture_which_resulted_in_explosion_of_our_king_is_prevented(
        ) {
            let fen = "3K4/8/8/8/7n/8/6N1/7k b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(H1, H2)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H4, F3)));
            assert!(moves.contains(&Move::from_to(H4, F5)));
            assert!(moves.contains(&Move::from_to(H4, G6)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_where_a_capture_which_resulted_in_explosion_of_our_king_is_prevented(
        ) {
            let fen = "3k4/8/8/8/7N/8/6n1/7K w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(H1, H2)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H4, F3)));
            assert!(moves.contains(&Move::from_to(H4, F5)));
            assert!(moves.contains(&Move::from_to(H4, G6)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_when_kings_touch() {
            let fen = "3K4/4k3/8/8/8/8/8/N6n b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 9);
            assert!(moves.contains(&Move::from_to(E7, E8)));
            assert!(moves.contains(&Move::from_to(E7, F8)));
            assert!(moves.contains(&Move::from_to(E7, F7)));
            assert!(moves.contains(&Move::from_to(E7, F6)));
            assert!(moves.contains(&Move::from_to(E7, E6)));
            assert!(moves.contains(&Move::from_to(E7, D6)));
            assert!(moves.contains(&Move::from_to(E7, D7)));
            assert!(moves.contains(&Move::from_to(H1, F2)));
            assert!(moves.contains(&Move::from_to(H1, G3)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_when_kings_touch() {
            let fen = "3k4/4K3/8/8/8/8/8/n6N w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 9);
            assert!(moves.contains(&Move::from_to(E7, E8)));
            assert!(moves.contains(&Move::from_to(E7, F8)));
            assert!(moves.contains(&Move::from_to(E7, F7)));
            assert!(moves.contains(&Move::from_to(E7, F6)));
            assert!(moves.contains(&Move::from_to(E7, E6)));
            assert!(moves.contains(&Move::from_to(E7, D6)));
            assert!(moves.contains(&Move::from_to(E7, D7)));
            assert!(moves.contains(&Move::from_to(H1, F2)));
            assert!(moves.contains(&Move::from_to(H1, G3)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_black_when_kings_touch_and_their_queen_gives_check(
        ) {
            let fen = "3K4/4k3/4Q3/8/8/8/8/7n b - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(E7, E8)));
            assert!(moves.contains(&Move::from_to(E7, F8)));
            assert!(moves.contains(&Move::from_to(E7, D7)));
            assert!(moves.contains(&Move::from_to(H1, F2)));
            assert!(moves.contains(&Move::from_to(H1, G3)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_when_kings_touch_and_their_queen_gives_check(
        ) {
            let fen = "3k4/4K3/4q3/8/8/8/8/7N w - - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(E7, E8)));
            assert!(moves.contains(&Move::from_to(E7, F8)));
            assert!(moves.contains(&Move::from_to(E7, D7)));
            assert!(moves.contains(&Move::from_to(H1, F2)));
            assert!(moves.contains(&Move::from_to(H1, G3)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_and_prevents_castling_because_in_check(
        ) {
            let fen = "4k3/8/8/8/8/8/2n5/R3K2R w KQ - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 5);
            assert!(moves.contains(&Move::from_to(E1, D1)));
            assert!(moves.contains(&Move::from_to(E1, D2)));
            assert!(moves.contains(&Move::from_to(E1, E2)));
            assert!(moves.contains(&Move::from_to(E1, F2)));
            assert!(moves.contains(&Move::from_to(E1, F1)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_and_prevents_castling_because_squares_are_attacked(
        ) {
            let fen = "4k3/8/8/8/8/p3n2p/P6P/R3K2R w KQ - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 8);
            assert!(moves.contains(&Move::from_to(E1, D2)));
            assert!(moves.contains(&Move::from_to(E1, E2)));
            assert!(moves.contains(&Move::from_to(E1, F2)));
            assert!(moves.contains(&Move::from_to(A1, B1)));
            assert!(moves.contains(&Move::from_to(A1, C1)));
            assert!(moves.contains(&Move::from_to(A1, D1)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, F1)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_and_prevents_long_castling_because_squares_are_attacked(
        ) {
            let fen = "2r1k3/8/8/8/8/p6p/P6P/R3K2R w KQ - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 11);
            assert!(moves.contains(&Move::from_to(E1, D2)));
            assert!(moves.contains(&Move::from_to(E1, E2)));
            assert!(moves.contains(&Move::from_to(E1, F2)));
            assert!(moves.contains(&Move::from_to(E1, D1)));
            assert!(moves.contains(&Move::from_to(E1, F1)));
            assert!(moves.contains(&Move::castle_short_white()));
            assert!(moves.contains(&Move::from_to(A1, B1)));
            assert!(moves.contains(&Move::from_to(A1, C1)));
            assert!(moves.contains(&Move::from_to(A1, D1)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, F1)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_and_prevents_short_castling_because_squares_are_attacked(
        ) {
            let fen = "4k1r1/8/8/8/8/p6p/P6P/R3K2R w KQ - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 11);
            assert!(moves.contains(&Move::from_to(E1, D2)));
            assert!(moves.contains(&Move::from_to(E1, E2)));
            assert!(moves.contains(&Move::from_to(E1, F2)));
            assert!(moves.contains(&Move::from_to(E1, D1)));
            assert!(moves.contains(&Move::from_to(E1, F1)));
            assert!(moves.contains(&Move::castle_long_white()));
            assert!(moves.contains(&Move::from_to(A1, B1)));
            assert!(moves.contains(&Move::from_to(A1, C1)));
            assert!(moves.contains(&Move::from_to(A1, D1)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, F1)));
        }

        #[test]
        fn it_generates_all_legal_moves_for_white_and_allows_short_castling_because_kings_touch(
        ) {
            let fen = "6r1/8/8/8/8/p6p/P4k1P/R3K2R w KQ - 0 1";
            let mut board = Board::from_fen(fen);
            let moves = generate_moves(&mut board);

            assert_eq!(moves.len(), 11);
            assert!(moves.contains(&Move::from_to(E1, D2)));
            assert!(moves.contains(&Move::from_to(E1, E2)));
            assert!(moves.contains(&Move::from_to(E1, D1)));
            assert!(moves.contains(&Move::from_to(E1, F1)));
            assert!(moves.contains(&Move::castle_long_white()));
            assert!(moves.contains(&Move::castle_short_white()));
            assert!(moves.contains(&Move::from_to(A1, B1)));
            assert!(moves.contains(&Move::from_to(A1, C1)));
            assert!(moves.contains(&Move::from_to(A1, D1)));
            assert!(moves.contains(&Move::from_to(H1, G1)));
            assert!(moves.contains(&Move::from_to(H1, F1)));
        }
    }
}

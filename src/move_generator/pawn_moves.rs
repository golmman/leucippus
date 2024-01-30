use std::ops::Range;

use crate::model::board::Board;
use crate::model::color::Color;
use crate::model::r#move::Move;
use crate::model::types::SquareIndex;
use crate::model::types::EN_PASSANT_CANDIDATES;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    let (forward, rank27, west_capture, east_capture) = match board.our_color {
        Color::Black => (-8, (48..56), -9, -7),
        Color::White => (8, (8..16), 7, 9),
    };

    for from in &board.pieces.our_pawns {
        add_forwards(board, &mut moves, *from, forward, &rank27);
        add_west_captures(board, &mut moves, *from, west_capture);
        add_east_captures(board, &mut moves, *from, east_capture);
    }

    add_en_passants(board, &mut moves);

    moves
}

fn add_en_passants(board: &Board, moves: &mut Vec<Move>) {
    let Some(to) = board.en_passant else {
        return;
    };

    if let (Some(west), ..) = EN_PASSANT_CANDIDATES[to as usize] {
        if let Some(piece) = board.pieces.squares.data[west as usize] {
            if piece.is_pawn_of_color(board.our_color) {
                moves.push(Move::en_passant(west, to));
            }
        }
    }

    if let (.., Some(east)) = EN_PASSANT_CANDIDATES[to as usize] {
        if let Some(piece) = board.pieces.squares.data[east as usize] {
            if piece.is_pawn_of_color(board.our_color) {
                moves.push(Move::en_passant(east, to));
            }
        }
    }
}

fn add_west_captures(
    board: &Board,
    moves: &mut Vec<Move>,
    from: SquareIndex,
    west_capture: i8,
) {
    if from % 8 == 0 {
        return;
    }

    let to = (from as i8 + west_capture) as u8;

    if let Some(piece) = board.pieces.squares.data[to as usize] {
        if piece.get_color() != board.our_color {
            moves.push(Move::from_to(from, to));
        }
    }
}

fn add_east_captures(
    board: &Board,
    moves: &mut Vec<Move>,
    from: SquareIndex,
    east_capture: i8,
) {
    if from % 8 == 7 {
        return;
    }

    let to = (from as i8 + east_capture) as u8;

    if let Some(piece) = board.pieces.squares.data[to as usize] {
        if piece.get_color() != board.our_color {
            moves.push(Move::from_to(from, to));
        }
    }
}

fn add_forwards(
    board: &Board,
    moves: &mut Vec<Move>,
    from: SquareIndex,
    forward: i8,
    rank27: &Range<u8>,
) {
    let to = (from as i8 + forward) as u8;

    if board.pieces.squares.data[to as usize].is_some() {
        return;
    }

    // promotions
    if to >= 56 {
        moves.push(Move::promote_bishop_white(from, to));
        moves.push(Move::promote_knight_white(from, to));
        moves.push(Move::promote_queen_white(from, to));
        moves.push(Move::promote_rook_white(from, to));
        return;
    }
    if to < 8 {
        moves.push(Move::promote_bishop_black(from, to));
        moves.push(Move::promote_knight_black(from, to));
        moves.push(Move::promote_queen_black(from, to));
        moves.push(Move::promote_rook_black(from, to));
        return;
    }

    // single step forward
    moves.push(Move::from_to(from, to));

    // double step forward
    let to = (from as i8 + forward + forward) as u8;

    if rank27.contains(&from) {
        if board.pieces.squares.data[to as usize].is_none() {
            moves.push(Move::from_to(from, to));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_white_pawn_moves_unless_blocked() {
        let fen = "2N4n/2P4P/5n2/4bP2/Nn6/3N2r1/PP1PP1P1/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves,
            vec![
                Move::from_to(8, 16),
                Move::from_to(9, 17),
                Move::from_to(12, 20),
                Move::from_to(12, 28),
            ]
        );
    }

    #[test]
    fn it_generates_black_starting_position_pawn_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 16);
        assert_eq!(
            moves,
            vec![
                Move::from_to(48, 40),
                Move::from_to(48, 32),
                Move::from_to(49, 41),
                Move::from_to(49, 33),
                Move::from_to(50, 42),
                Move::from_to(50, 34),
                Move::from_to(51, 43),
                Move::from_to(51, 35),
                Move::from_to(52, 44),
                Move::from_to(52, 36),
                Move::from_to(53, 45),
                Move::from_to(53, 37),
                Move::from_to(54, 46),
                Move::from_to(54, 38),
                Move::from_to(55, 47),
                Move::from_to(55, 39),
            ]
        );
    }

    #[test]
    fn it_generates_white_starting_position_pawn_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 16);
        assert_eq!(
            moves,
            vec![
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
    fn it_generates_unblocked_black_forward_moves() {
        let fen = "8/7p/6p1/5p2/4p3/3p4/2p5/8 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 10);
        assert_eq!(
            moves,
            vec![
                Move::promote_bishop_black(10, 2),
                Move::promote_knight_black(10, 2),
                Move::promote_queen_black(10, 2),
                Move::promote_rook_black(10, 2),
                Move::from_to(19, 11),
                Move::from_to(28, 20),
                Move::from_to(37, 29),
                Move::from_to(46, 38),
                Move::from_to(55, 47),
                Move::from_to(55, 39),
            ]
        );
    }

    #[test]
    fn it_generates_unblocked_white_forward_moves() {
        let fen = "8/5P2/4P3/3P4/2P5/1P6/P7/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 10);
        assert_eq!(
            moves,
            vec![
                Move::from_to(8, 16),
                Move::from_to(8, 24),
                Move::from_to(17, 25),
                Move::from_to(26, 34),
                Move::from_to(35, 43),
                Move::from_to(44, 52),
                Move::promote_bishop_white(53, 61),
                Move::promote_knight_white(53, 61),
                Move::promote_queen_white(53, 61),
                Move::promote_rook_white(53, 61),
            ]
        );
    }

    #[test]
    fn it_generates_all_captures_for_white() {
        let fen = "8/8/8/pppppppp/PPPPPPPP/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 14);
        assert_eq!(
            moves,
            vec![
                Move::from_to(24, 33),
                Move::from_to(25, 32),
                Move::from_to(25, 34),
                Move::from_to(26, 33),
                Move::from_to(26, 35),
                Move::from_to(27, 34),
                Move::from_to(27, 36),
                Move::from_to(28, 35),
                Move::from_to(28, 37),
                Move::from_to(29, 36),
                Move::from_to(29, 38),
                Move::from_to(30, 37),
                Move::from_to(30, 39),
                Move::from_to(31, 38),
            ]
        );
    }

    #[test]
    fn it_generates_all_captures_for_black() {
        let fen = "8/8/8/pppppppp/PPPPPPPP/8/8/8 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 14);
        assert_eq!(
            moves,
            vec![
                Move::from_to(32, 25),
                Move::from_to(33, 24),
                Move::from_to(33, 26),
                Move::from_to(34, 25),
                Move::from_to(34, 27),
                Move::from_to(35, 26),
                Move::from_to(35, 28),
                Move::from_to(36, 27),
                Move::from_to(36, 29),
                Move::from_to(37, 28),
                Move::from_to(37, 30),
                Move::from_to(38, 29),
                Move::from_to(38, 31),
                Move::from_to(39, 30),
            ]
        );
    }

    #[test]
    fn it_generates_all_promotion_captures_for_white() {
        let fen = "nnnnnnnn/PPPPPPPP/8/8/8/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn it_generates_all_promotion_captures_for_black() {
        let fen = "8/8/8/8/8/8/pppppppp/NNNNNNNN b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 14);
    }

    #[test]
    fn it_generates_captures_for_white_unless_wrong_color() {
        let fen = "4Nnn1/5P2/8/3NNN2/4P3/1NNN4/2P5/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves, vec![Move::from_to(53, 62),]);
    }

    #[test]
    fn it_generates_captures_for_black_unless_wrong_color() {
        let fen = "8/2p5/1nnn4/4p3/3nnn2/8/2p5/1Nnn4 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves, vec![Move::from_to(10, 1),]);
    }

    mod en_passants {
        use crate::model::types::square_names::*;

        use super::*;

        #[test]
        fn it_generates_one_en_passant_for_black() {
            let fen =
                "rnbqkbnr/1ppppppp/8/8/pP5P/8/P1PPPPP1/RNBQKBNR b KQkq b3 0 3";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(1 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(A4, B3)));
        }

        #[test]
        fn it_generates_one_en_passant_for_white() {
            let fen =
                "rnbqkbnr/pppp2pp/8/3Ppp2/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(1 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(D5, E6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_1() {
            let fen =
                "rnbqkbnr/p1pppppp/8/8/Pp5P/8/1PPPPPP1/RNBQKBNR b KQkq a3 0 3";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(1 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(B4, A3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_2() {
            let fen =
                "rnbqkbnr/1p1ppppp/8/7P/pPp5/6P1/P1PPPP2/RNBQKBNR b KQkq b3 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(A4, B3)));
            assert!(moves.contains(&Move::en_passant(C4, B3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_3() {
            let fen =
                "rnbqkbnr/p1p1pppp/7P/8/1pPp4/8/PP1PPPP1/RNBQKBNR b KQkq c3 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(B4, C3)));
            assert!(moves.contains(&Move::en_passant(D4, C3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_4() {
            let fen =
                "rnbqkbnr/pp1p1ppp/7P/8/2pPp3/8/PPP1PPP1/RNBQKBNR b KQkq d3 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(C4, D3)));
            assert!(moves.contains(&Move::en_passant(E4, D3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_5() {
            let fen =
                "rnbqkbnr/ppp1p1pp/7P/8/3pPp2/8/PPPP1PP1/RNBQKBNR b KQkq e3 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(D4, E3)));
            assert!(moves.contains(&Move::en_passant(F4, E3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_6() {
            let fen =
                "rnbqkbnr/pppp1p1p/7P/8/4pPp1/8/PPPPP1P1/RNBQKBNR b KQkq f3 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(E4, F3)));
            assert!(moves.contains(&Move::en_passant(G4, F3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_7() {
            let fen =
                "rnbqkbnr/ppppp1p1/8/P7/5pPp/7P/1PPPPP2/RNBQKBNR b KQkq g3 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(F4, G3)));
            assert!(moves.contains(&Move::en_passant(H4, G3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_black_8() {
            let fen =
                "rnbqkbnr/pppppp1p/8/8/P5pP/8/1PPPPPP1/RNBQKBNR b KQkq h3 0 3";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(1 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(G4, H3)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_1() {
            let fen =
                "rnbqkbnr/1pppppp1/8/pP5p/8/8/P1PPPPPP/RNBQKBNR w KQkq a6 0 3";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(1 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(B5, A6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_2() {
            let fen =
                "rnbqkbnr/p1ppppp1/8/PpP5/8/7p/1P1PPPPP/RNBQKBNR w KQkq b6 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(A5, B6)));
            assert!(moves.contains(&Move::en_passant(C5, B6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_3() {
            let fen =
                "rnbqkbnr/pp1pppp1/8/1PpP4/7p/8/P1P1PPPP/RNBQKBNR w KQkq c6 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(B5, C6)));
            assert!(moves.contains(&Move::en_passant(D5, C6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_4() {
            let fen =
                "rnbqkbnr/ppp1ppp1/8/2PpP3/7p/8/PP1P1PPP/RNBQKBNR w KQkq d6 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(C5, D6)));
            assert!(moves.contains(&Move::en_passant(E5, D6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_5() {
            let fen =
                "rnbqkbnr/pppp1pp1/8/3PpP2/7p/8/PPP1P1PP/RNBQKBNR w KQkq e6 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(D5, E6)));
            assert!(moves.contains(&Move::en_passant(F5, E6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_6() {
            let fen =
                "rnbqkbnr/1pppp1pp/8/4PpP1/p7/8/PPPP1P1P/RNBQKBNR w KQkq f6 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(E5, F6)));
            assert!(moves.contains(&Move::en_passant(G5, F6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_7() {
            let fen =
                "rnbqkbnr/1ppppp1p/8/5PpP/p7/8/PPPPP1P1/RNBQKBNR w KQkq g6 0 5";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(2 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(F5, G6)));
            assert!(moves.contains(&Move::en_passant(H5, G6)));
        }

        #[test]
        fn it_generates_all_en_passants_for_white_8() {
            let fen =
                "rnbqkbnr/1pppppp1/p7/6Pp/8/8/PPPPPP1P/RNBQKBNR w KQkq h6 0 3";
            let board = Board::from_fen(fen);
            let moves = generate(&board);
            assert!(1 == moves.iter().filter(|m| m.is_en_passant()).count());
            assert!(moves.contains(&Move::en_passant(G5, H6)));
        }
    }
}

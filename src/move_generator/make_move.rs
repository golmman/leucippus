use crate::model::board::Board;
use crate::model::color::Color;
use crate::model::piece::Piece;
use crate::model::r#move::Move;
use crate::model::r#move::MoveSpecial;
use crate::model::types::square_names::*;
use crate::model::types::SquareIndex;
use crate::model::types::SQUARE_NEIGHBORHOODS;

pub fn make_move(board: &mut Board, m: &Move) {
    if m.is_castle() {
        make_castle(board, m);
    } else if m.is_promotion() {
        make_promotion(board, m);
    } else if m.is_en_passant() {
        make_en_passant(board, m);
    } else if is_capture(board, m) {
        make_capture(board, m);
    } else {
        make_simple_move(board, m);
    }

    update_board_state(board, m);
}

pub fn make_null_move(board: &mut Board) {
    board.swap_color();

    board.pieces.active_bishops.clear();
    board.pieces.active_kings.clear();
    board.pieces.active_knights.clear();
    board.pieces.active_pawns.clear();
    board.pieces.active_queens.clear();
    board.pieces.active_rooks.clear();

    for i in 0..64 {
        if let Some(piece) = board.pieces.squares.data[i] {
            if piece.is_bishop_of_color(board.color) {
                board.pieces.active_bishops.push(i as SquareIndex);
            } else if piece.is_king_of_color(board.color) {
                board.pieces.active_kings.push(i as SquareIndex);
            } else if piece.is_knight_of_color(board.color) {
                board.pieces.active_knights.push(i as SquareIndex);
            } else if piece.is_pawn_of_color(board.color) {
                board.pieces.active_pawns.push(i as SquareIndex);
            } else if piece.is_queen_of_color(board.color) {
                board.pieces.active_queens.push(i as SquareIndex);
            } else if piece.is_rook_of_color(board.color) {
                board.pieces.active_rooks.push(i as SquareIndex);
            }
        }
    }
}

fn make_simple_move(board: &mut Board, m: &Move) {
    let piece = board.pieces.squares.data[m.from as usize];
    board.pieces.squares.data[m.from as usize] = None;
    board.pieces.squares.data[m.to as usize] = piece;
}

fn make_castle(board: &mut Board, m: &Move) {
    match m.special {
        Some(MoveSpecial::CastleLongBlack) => {
            board.pieces.squares.data[A8 as usize] = None;
            board.pieces.squares.data[C8 as usize] = Some(Piece::BlackKing);
            board.pieces.squares.data[D8 as usize] = Some(Piece::BlackRook);
            board.pieces.squares.data[E8 as usize] = None;
        }
        Some(MoveSpecial::CastleLongWhite) => {
            board.pieces.squares.data[A1 as usize] = None;
            board.pieces.squares.data[C1 as usize] = Some(Piece::WhiteKing);
            board.pieces.squares.data[D1 as usize] = Some(Piece::WhiteRook);
            board.pieces.squares.data[E1 as usize] = None;
        }
        Some(MoveSpecial::CastleShortBlack) => {
            board.pieces.squares.data[E8 as usize] = None;
            board.pieces.squares.data[F8 as usize] = Some(Piece::BlackRook);
            board.pieces.squares.data[G8 as usize] = Some(Piece::BlackKing);
            board.pieces.squares.data[H8 as usize] = None;
        }
        Some(MoveSpecial::CastleShortWhite) => {
            board.pieces.squares.data[E1 as usize] = None;
            board.pieces.squares.data[F1 as usize] = Some(Piece::WhiteRook);
            board.pieces.squares.data[G1 as usize] = Some(Piece::WhiteKing);
            board.pieces.squares.data[H1 as usize] = None;
        }
        _ => panic!("Move should be castling"),
    }
}

fn make_capture(board: &mut Board, m: &Move) {
    board.pieces.squares.data[m.from as usize] = None;
    board.pieces.squares.data[m.to as usize] = None;

    for blast in SQUARE_NEIGHBORHOODS[m.to as usize] {
        let Some(blast) = blast else {
            break;
        };
        if !board.has_pawn_at(blast) {
            board.pieces.squares.data[blast as usize] = None;
        }
    }
}

fn make_en_passant(board: &mut Board, m: &Move) {
    make_capture(board, m);

    if board.color == Color::Black {
        board.pieces.squares.data[(m.to + 8) as usize] = None;
    } else {
        board.pieces.squares.data[(m.to - 8) as usize] = None;
    }
}

fn make_promotion(board: &mut Board, m: &Move) {
    match m.special {
        Some(MoveSpecial::PromoteBishopBlack) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::BlackBishop);
        }
        Some(MoveSpecial::PromoteKnightBlack) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::BlackKnight);
        }
        Some(MoveSpecial::PromoteQueenBlack) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::BlackQueen);
        }
        Some(MoveSpecial::PromoteRookBlack) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::BlackRook);
        }
        Some(MoveSpecial::PromoteBishopWhite) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::WhiteBishop);
        }
        Some(MoveSpecial::PromoteKnightWhite) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::WhiteKnight);
        }
        Some(MoveSpecial::PromoteQueenWhite) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::WhiteQueen);
        }
        Some(MoveSpecial::PromoteRookWhite) => {
            board.pieces.squares.data[m.from as usize] = None;
            board.pieces.squares.data[m.to as usize] = Some(Piece::WhiteRook);
        }
        _ => panic!("Move should be castling"),
    }
}

fn is_capture(board: &mut Board, m: &Move) -> bool {
    board.pieces.squares.data[m.to as usize]
        .is_some_and(|p| p.get_color() != board.color)
}

fn was_double_pawn_advance(board: &mut Board, m: &Move) -> bool {
    board.pieces.squares.data[m.to as usize].is_some_and(Piece::is_pawn)
        && (m.from as i8 - m.to as i8).abs() == 16
}

fn was_capture_or_pawn_advance(board: &mut Board, m: &Move) -> bool {
    board.pieces.squares.data[m.to as usize].is_none()
        || board.pieces.squares.data[m.to as usize].is_some_and(Piece::is_pawn)
        || m.is_promotion()
}

fn update_board_state(board: &mut Board, m: &Move) {
    // color and fullmove
    if board.color == Color::Black {
        board.color = Color::White;
        board.fullmove += 1;
    } else {
        board.color = Color::Black;
    }

    // halfmove
    if was_capture_or_pawn_advance(board, m) {
        board.halfmove = 0;
    } else {
        board.halfmove += 1;
    }

    // en_passant
    if was_double_pawn_advance(board, m) {
        let en_passant = Some((m.from + m.to) / 2);

        if board.has_pawn_of_color_at(board.color, m.to - 1)
            && m.to != A4
            && m.to != A5
        {
            board.en_passant = en_passant;
        }
        if board.has_pawn_of_color_at(board.color, m.to + 1)
            && m.to != H4
            && m.to != H5
        {
            board.en_passant = en_passant;
        }
    } else {
        board.en_passant = None;
    }

    // castle
    if board.is_empty_at(0) {
        board.castle.white_long = false;
    }
    if board.is_empty_at(7) {
        board.castle.white_short = false;
    }
    if board.is_empty_at(56) {
        board.castle.black_long = false;
    }
    if board.is_empty_at(63) {
        board.castle.black_short = false;
    }
    if m.from == 4 {
        board.castle.white_long = false;
        board.castle.white_short = false;
    } else if m.from == 60 {
        board.castle.black_long = false;
        board.castle.black_short = false;
    }

    // active pieces
    board.pieces.active_bishops.clear();
    board.pieces.active_kings.clear();
    board.pieces.active_knights.clear();
    board.pieces.active_pawns.clear();
    board.pieces.active_queens.clear();
    board.pieces.active_rooks.clear();

    for i in 0..64 {
        if let Some(piece) = board.pieces.squares.data[i] {
            if piece.is_bishop_of_color(board.color) {
                board.pieces.active_bishops.push(i as SquareIndex);
            } else if piece.is_king_of_color(board.color) {
                board.pieces.active_kings.push(i as SquareIndex);
            } else if piece.is_knight_of_color(board.color) {
                board.pieces.active_knights.push(i as SquareIndex);
            } else if piece.is_pawn_of_color(board.color) {
                board.pieces.active_pawns.push(i as SquareIndex);
            } else if piece.is_queen_of_color(board.color) {
                board.pieces.active_queens.push(i as SquareIndex);
            } else if piece.is_rook_of_color(board.color) {
                board.pieces.active_rooks.push(i as SquareIndex);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::types::square_names::*;

    #[test]
    fn it_makes_a_null_move_at_the_starting_position() {
        let mut board = Board::new();
        make_null_move(&mut board);
        assert_eq!(board, Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1"));
    }

    #[test]
    fn it_proves_null_moves_are_idempotent() {
        let mut board = Board::new();
        make_null_move(&mut board);
        make_null_move(&mut board);
        assert_eq!(board, Board::new());
    }

    #[test]
    fn it_makes_correct_moves_in_a_sample_game() {
        let fens = vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            "rnbqkbnr/ppppppp1/7p/8/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 2",
            "rnbqkbnr/ppppppp1/7p/8/8/2N4N/PPPPPPPP/R1BQKB1R b KQkq - 1 2",
            "rnbqkbnr/pp1pppp1/2p4p/8/8/2N4N/PPPPPPPP/R1BQKB1R w KQkq - 0 3",
            "rnbqkbnr/pp1pppp1/2p4p/8/3P4/2N4N/PPP1PPPP/R1BQKB1R b KQkq - 0 3",
            "rnbqkbnr/pp1p1pp1/2p1p2p/8/3P4/2N4N/PPP1PPPP/R1BQKB1R w KQkq - 0 4",
            "rnbqkbnr/pp1p1pp1/2p1p2p/8/3PP3/2N4N/PPP2PPP/R1BQKB1R b KQkq - 0 4",
            "rnbqk1nr/pp1p1pp1/2p1p2p/8/1b1PP3/2N4N/PPP2PPP/R1BQKB1R w KQkq - 1 5",
            "rnbqk1nr/pp1p1pp1/2p1p2p/8/1b1PP3/2N4N/PPP1BPPP/R1BQK2R b KQkq - 2 5",
            "rnbqk1nr/pp3pp1/2p1p2p/3p4/1b1PP3/2N4N/PPP1BPPP/R1BQK2R w KQkq - 0 6",
            "rnbqk1nr/pp3pp1/2p1p2p/3p2N1/1b1PP3/2N5/PPP1BPPP/R1BQK2R b KQkq - 1 6",
            "rnbqk1nr/pp3pp1/2p1p3/3p4/1b1PP3/2N5/PPP1BPPP/R1BQK2R w KQkq - 0 7",
            "rnbqk1nr/pp3pp1/2p1p3/3p4/1b1PP3/2N3P1/PPP1BP1P/R1BQK2R b KQkq - 0 7",
            "rnbqk1n1/pp3pp1/2p1p3/3p4/1b1PP3/2N3P1/PPP1BP2/R1BQK3 w Qq - 0 8",
            "rnbqk1n1/pp3pp1/2p1p3/3p3B/1b1PP3/2N3P1/PPP2P2/R1BQK3 b Qq - 1 8",
            "rnbqk1n1/pp3p2/2p1p1p1/3p3B/1b1PP3/2N3P1/PPP2P2/R1BQK3 w Qq - 0 9",
            "rnbqk1n1/pp3p2/2p1p1p1/3p4/1b1PP3/2N3P1/PPP1BP2/R1BQK3 b Qq - 1 9",
            "rnbqk3/pp3p2/2p1p1pn/3p4/1b1PP3/2N3P1/PPP1BP2/R1BQK3 w Qq - 2 10",
            "rnbqk3/pp3p2/2p1p1pn/3p4/1b1PP1P1/2N5/PPP1BP2/R1BQK3 b Qq - 0 10",
            "rnb1k3/pp3p2/2p1p1pn/3p4/1b1PP1Pq/2N5/PPP1BP2/R1BQK3 w Qq - 1 11",
            "rnb1k3/pp3p2/2p1p1pn/3p4/1b1PPBPq/2N5/PPP1BP2/R2QK3 b Qq - 2 11",
            "rnb1k3/pp3p2/2p1p1pn/3p4/1b1PPBP1/2N5/PPP5/R2Q4 w Qq - 0 12",
        ];

        let moves = vec![
            Move::from_to(G1, H3),
            Move::from_to(H7, H6),
            Move::from_to(B1, C3),
            Move::from_to(C7, C6),
            Move::from_to(D2, D4),
            Move::from_to(E7, E6),
            Move::from_to(E2, E4),
            Move::from_to(F8, B4),
            Move::from_to(F1, E2),
            Move::from_to(D7, D5),
            Move::from_to(H3, G5),
            Move::from_to(H6, G5),
            Move::from_to(G2, G3),
            Move::from_to(H8, H2),
            Move::from_to(E2, H5),
            Move::from_to(G7, G6),
            Move::from_to(H5, E2),
            Move::from_to(G8, H6),
            Move::from_to(G3, G4),
            Move::from_to(D8, H4),
            Move::from_to(C1, F4),
            Move::from_to(H4, F2),
        ];

        for i in 0..moves.len() {
            let mut before = Board::from_fen(fens[i]);
            let after = Board::from_fen(fens[i + 1]);
            make_move(&mut before, &moves[i]);
            assert_eq!(before, after);
        }
    }

    mod en_passant_moves {
        use super::*;

        #[test]
        fn black_makes_an_en_passant_capture() {
            let mut board = Board::from_fen(
                "rnbqkbnr/1pppp1pp/8/p7/3PPp2/3Q1N2/PPP2PPP/RNB1KB1R b KQkq e3 0 4",
            );
            make_move(&mut board, &Move::en_passant(F4, E3));
            assert_eq!(
                board,
                Board::from_fen("rnbqkbnr/1pppp1pp/8/p7/3P4/8/PPP2PPP/RNB1KB1R w KQkq - 0 5")
            );
        }

        #[test]
        fn white_makes_an_en_passant_capture() {
            let mut board = Board::from_fen(
                "r1bqkbnr/ppp1pppp/2n5/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            );
            make_move(&mut board, &Move::en_passant(E5, D6));
            assert_eq!(
                board,
                Board::from_fen(
                    "r1bqkbnr/ppp1pppp/8/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3"
                )
            );
        }
    }

    mod capture_moves {
        use super::*;

        #[test]
        fn white_causes_maximal_destruction() {
            let mut board = Board::from_fen(
                "8/1kqqQr2/2qqQ3/2qqqn2/2N5/8/2B5/2K5 w - - 0 1",
            );
            make_move(&mut board, &Move::from_to(C4, D6));
            assert_eq!(
                board,
                Board::from_fen("8/1k3r2/8/5n2/8/8/2B5/2K5 b - - 0 1")
            );
        }

        #[test]
        fn black_explodes_the_white_king() {
            let mut board = Board::from_fen(
                "rnbqkb1r/pppppppp/8/P7/4n3/8/1PPPPPPP/RNBQKBNR b KQkq - 0 3",
            );
            make_move(&mut board, &Move::from_to(E4, D2));
            assert_eq!(
                board,
                Board::from_fen(
                    "rnbqkb1r/pppppppp/8/P7/8/8/1PP1PPPP/RN3BNR w KQkq - 0 4"
                )
            );
        }

        #[test]
        fn black_jumps_into_a_pawn_clump_but_only_the_center_pawn_gets_destroyed(
        ) {
            let mut board =
                Board::from_fen("6k1/8/2PpP3/2pPp3/2PpP3/2n5/8/6K1 b - - 0 1");
            make_move(&mut board, &Move::from_to(C3, D5));
            assert_eq!(
                board,
                Board::from_fen("6k1/8/2PpP3/2p1p3/2PpP3/8/8/6K1 w - - 0 2")
            );
        }
    }

    mod promotion_moves {
        use super::*;

        #[test]
        fn black_promotes_to_bishop() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/pP3PPP/3R1RK1 b - - 0 19",
            );
            make_move(&mut board, &Move::promote_bishop_black(A2, A1));
            assert_eq!(
                board,
                Board::from_fen(
                    "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 w - - 0 20"
                )
            );
        }

        #[test]
        fn black_promotes_to_knight() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/pP3PPP/3R1RK1 b - - 0 19",
            );
            make_move(&mut board, &Move::promote_knight_black(A2, A1));
            assert_eq!(
                board,
                Board::from_fen(
                    "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/n2R1RK1 w - - 0 20"
                )
            );
        }

        #[test]
        fn black_promotes_to_queen() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/pP3PPP/3R1RK1 b - - 0 19",
            );
            make_move(&mut board, &Move::promote_queen_black(A2, A1));
            assert_eq!(
                board,
                Board::from_fen(
                    "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/q2R1RK1 w - - 0 20"
                )
            );
        }

        #[test]
        fn black_promotes_to_rook() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/pP3PPP/3R1RK1 b - - 0 19",
            );
            make_move(&mut board, &Move::promote_rook_black(A2, A1));
            assert_eq!(
                board,
                Board::from_fen(
                    "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/r2R1RK1 w - - 0 20"
                )
            );
        }

        #[test]
        fn white_promotes_to_bishop() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 w - - 0 20",
            );
            make_move(&mut board, &Move::promote_bishop_white(C7, C8));
            assert_eq!(
                board,
                Board::from_fen(
                    "2B3k1/1p3ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 b - - 0 20"
                )
            );
        }

        #[test]
        fn white_promotes_to_knight() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 w - - 0 20",
            );
            make_move(&mut board, &Move::promote_knight_white(C7, C8));
            assert_eq!(
                board,
                Board::from_fen(
                    "2N3k1/1p3ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 b - - 0 20"
                )
            );
        }

        #[test]
        fn white_promotes_to_queen() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 w - - 0 20",
            );
            make_move(&mut board, &Move::promote_queen_white(C7, C8));
            assert_eq!(
                board,
                Board::from_fen(
                    "2Q3k1/1p3ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 b - - 0 20"
                )
            );
        }

        #[test]
        fn white_promotes_to_rook() {
            let mut board = Board::from_fen(
                "6k1/1pP2ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 w - - 0 20",
            );
            make_move(&mut board, &Move::promote_rook_white(C7, C8));
            assert_eq!(
                board,
                Board::from_fen(
                    "2R3k1/1p3ppp/8/8/r3N3/5N2/1P3PPP/b2R1RK1 b - - 0 20"
                )
            );
        }
    }

    mod castle_moves {
        use super::*;

        #[test]
        fn white_castles_long() {
            let mut board = Board::from_fen(
                "r3k2r/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/R3K2R w KQkq - 10 8",
            );
            make_move(&mut board, &Move::castle_long_white());
            assert_eq!(board, Board::from_fen("r3k2r/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/2KR3R b kq - 11 8"));
        }

        #[test]
        fn white_castles_short() {
            let mut board = Board::from_fen(
                "r3k2r/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/R3K2R w KQkq - 10 8",
            );
            make_move(&mut board, &Move::castle_short_white());
            assert_eq!(board, Board::from_fen("r3k2r/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/R4RK1 b kq - 11 8"));
        }

        #[test]
        fn black_castles_long() {
            let mut board = Board::from_fen(
                "r3k2r/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/R4RK1 b kq - 11 8",
            );
            make_move(&mut board, &Move::castle_long_black());
            assert_eq!(board, Board::from_fen("2kr3r/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/R4RK1 w - - 12 9"));
        }

        #[test]
        fn black_castles_short() {
            let mut board = Board::from_fen(
                "r3k2r/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/R4RK1 b kq - 11 8",
            );
            make_move(&mut board, &Move::castle_short_black());
            assert_eq!(board, Board::from_fen("r4rk1/pppq1ppp/2n2n2/2bppb2/2BPPB2/2N2N2/PPPQ1PPP/R4RK1 w - - 12 9"));
        }
    }

    mod simple_moves {
        use super::*;

        // The following tests 'play' a simple game
        // PGN: "1. Nf3 f6 2. Rg1 b5 3. h4 b4 4. a4 Ba61. Nf3 f6 2. Rg1 b5 3. h4 b4 4. a4 Ba6"

        #[test]
        fn white_makes_a_knight_move_from_the_starting_position() {
            let mut board = Board::from_fen(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            );
            make_move(&mut board, &Move::from_to(G1, F3));
            assert_eq!(board, Board::from_fen("rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1"));
        }

        #[test]
        fn black_responds_with_a_pawn_move() {
            let mut board = Board::from_fen(
                "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            );
            make_move(&mut board, &Move::from_to(F7, F6));
            assert_eq!(board, Board::from_fen("rnbqkbnr/ppppp1pp/5p2/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2"));
        }

        #[test]
        fn white_loses_short_castle_rights_because_of_a_rook_move() {
            let mut board = Board::from_fen(
                "rnbqkbnr/ppppp1pp/5p2/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2",
            );
            make_move(&mut board, &Move::from_to(H1, G1));
            assert_eq!(board, Board::from_fen("rnbqkbnr/ppppp1pp/5p2/8/8/5N2/PPPPPPPP/RNBQKBR1 b Qkq - 1 2"));
        }

        #[test]
        fn black_makes_a_double_pawn_move() {
            let mut board = Board::from_fen(
                "rnbqkbnr/ppppp1pp/5p2/8/8/5N2/PPPPPPPP/RNBQKBR1 b Qkq - 1 2",
            );
            make_move(&mut board, &Move::from_to(B7, B5));
            assert_eq!(board, Board::from_fen("rnbqkbnr/p1ppp1pp/5p2/1p6/8/5N2/PPPPPPPP/RNBQKBR1 w Qkq - 0 3"));
        }

        #[test]
        fn white_makes_a_double_pawn_move() {
            let mut board = Board::from_fen(
                "rnbqkbnr/p1ppp1pp/5p2/1p6/8/5N2/PPPPPPPP/RNBQKBR1 w Qkq - 0 3",
            );
            make_move(&mut board, &Move::from_to(H2, H4));
            assert_eq!(board, Board::from_fen("rnbqkbnr/p1ppp1pp/5p2/1p6/7P/5N2/PPPPPPP1/RNBQKBR1 b Qkq - 0 3"));
        }

        #[test]
        fn black_pushes_their_pawn_further() {
            let mut board = Board::from_fen(
                "rnbqkbnr/p1ppp1pp/5p2/1p6/7P/5N2/PPPPPPP1/RNBQKBR1 b Qkq - 0 3",
            );
            make_move(&mut board, &Move::from_to(B5, B4));
            assert_eq!(board, Board::from_fen("rnbqkbnr/p1ppp1pp/5p2/8/1p5P/5N2/PPPPPPP1/RNBQKBR1 w Qkq - 0 4"));
        }

        #[test]
        fn white_makes_a_double_pawn_move_which_allows_en_passant() {
            let mut board = Board::from_fen(
                "rnbqkbnr/p1ppp1pp/5p2/8/1p5P/5N2/PPPPPPP1/RNBQKBR1 w Qkq - 0 4",
            );
            make_move(&mut board, &Move::from_to(A2, A4));
            assert_eq!(board, Board::from_fen("rnbqkbnr/p1ppp1pp/5p2/8/Pp5P/5N2/1PPPPPP1/RNBQKBR1 b Qkq a3 0 4"));
        }

        #[test]
        fn black_moves_their_bishop() {
            let mut board = Board::from_fen(
                "rnbqkbnr/p1ppp1pp/5p2/8/Pp5P/5N2/1PPPPPP1/RNBQKBR1 b Qkq a3 0 4",
            );
            make_move(&mut board, &Move::from_to(C8, A6));
            assert_eq!(board, Board::from_fen("rn1qkbnr/p1ppp1pp/b4p2/8/Pp5P/5N2/1PPPPPP1/RNBQKBR1 w Qkq - 1 5"));
        }

        #[test]
        fn white_makes_a_double_pawn_move_which_should_not_set_en_passant() {
            let mut board = Board::from_fen(
                "rn1qkbnr/p1ppp1pp/5p2/8/Ppb4P/1P3N2/2PPPPP1/RNBQKBR1 w Qkq - 1 6",
            );
            make_move(&mut board, &Move::from_to(D2, D4));
            assert_eq!(board, Board::from_fen("rn1qkbnr/p1ppp1pp/5p2/8/PpbP3P/1P3N2/2P1PPP1/RNBQKBR1 b Qkq - 0 6"));
        }
    }
}

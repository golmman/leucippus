use crate::model::board::Board;
use crate::model::color::Color;
use crate::model::piece::Piece;
use crate::model::r#move::Move;
use crate::model::r#move::MoveSpecial;
use crate::model::types::square_names::*;
use crate::model::types::SquareIndex;

pub fn make_move(board: &mut Board, m: &Move) {
    if m.is_castle() {
        make_castle(board, m);
    } else if m.is_en_passant() {
        make_en_passant(board, m);
    } else if m.is_promotion() {
        make_promotion(board, m);
    } else if is_capture(board, m) {
        make_capture(board, m);
    } else {
        make_simple_move(board, m);
    }

    update_board_state(board, m);
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
        _ => {}
    }
}

fn make_capture(board: &mut Board, m: &Move) {
    todo!()
}

fn make_en_passant(board: &mut Board, m: &Move) {
    todo!()
}

fn make_promotion(board: &mut Board, m: &Move) {
    todo!()
}

fn is_capture(board: &mut Board, m: &Move) -> bool {
    board.pieces.squares.data[m.to as usize]
        .as_ref()
        .map(Piece::get_color)
        .is_some_and(|c| c != board.color)
}

fn is_pawn_move(board: &mut Board, m: &Move) -> bool {
    board.pieces.squares.data[m.to as usize]
        .as_ref()
        .map_or(false, Piece::is_pawn)
}

fn update_board_state(board: &mut Board, m: &Move) {
    let capture = is_capture(board, m);
    let pawn_move = is_pawn_move(board, m);

    // color and fullmove
    if board.color == Color::Black {
        board.color = Color::White;
        board.fullmove += 1;
    } else {
        board.color = Color::Black;
    }

    // halfmove
    if capture || pawn_move {
        board.halfmove = 0;
    } else {
        board.halfmove += 1;
    }

    // en_passant
    if pawn_move && (m.from as i8 - m.to as i8).abs() == 16 {
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
    if m.from == 0 || m.to == 0 {
        board.castle.white_long = false;
    } else if m.from == 4 {
        board.castle.white_long = false;
        board.castle.white_short = false;
    } else if m.from == 7 || m.to == 7 {
        board.castle.white_short = false;
    } else if m.from == 56 || m.to == 56 {
        board.castle.black_long = false;
    } else if m.from == 60 {
        board.castle.black_long = false;
        board.castle.black_short = false;
    } else if m.from == 63 || m.to == 63 {
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
    }
}

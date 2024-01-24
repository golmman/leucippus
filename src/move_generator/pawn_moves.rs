use std::ops::Range;

use crate::model::board::Board;
use crate::model::color::Color;
use crate::model::r#move::Move;
use crate::model::r#move::MoveSpecial;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    let (forward, rank27, west_capture, east_capture) = match board.color {
        Color::Black => (-8, (48..56), -9, -7),
        Color::White => (8, (8..16), 7, 9),
    };

    for from in &board.pieces.active_pawns {
        let from_x = (from % 8) as i32;
        let from_y = (from / 8) as i32;

        add_forward_moves(board, &mut moves, *from, forward, &rank27);
        add_west_captures_moves(board, &mut moves, *from, west_capture);
        add_east_captures_moves(board, &mut moves, *from, east_capture);
    }

    moves
}

fn add_west_captures_moves(
    board: &Board,
    moves: &mut Vec<Move>,
    from: u8,
    west_capture: i8,
) {
    if from % 8 == 0 {
        return;
    }

    let to = (from as i8 + west_capture) as u8;

    if let Some(piece) = board.pieces.squares.data[to as usize] {
        if piece.get_color() != board.color {
            if to >= 56 || to < 8 {
                moves.push(Move::promote_bishop(from, to));
                moves.push(Move::promote_knight(from, to));
                moves.push(Move::promote_queen(from, to));
                moves.push(Move::promote_rook(from, to));
            } else {
                moves.push(Move::from_to(from, to));
            }
        }
    }
}

fn add_east_captures_moves(
    board: &Board,
    moves: &mut Vec<Move>,
    from: u8,
    east_capture: i8,
) {
    if from % 8 == 7 {
        return;
    }

    let to = (from as i8 + east_capture) as u8;

    if let Some(piece) = board.pieces.squares.data[to as usize] {
        if piece.get_color() != board.color {
            if to >= 56 || to < 8 {
                moves.push(Move::promote_bishop(from, to));
                moves.push(Move::promote_knight(from, to));
                moves.push(Move::promote_queen(from, to));
                moves.push(Move::promote_rook(from, to));
            } else {
                moves.push(Move::from_to(from, to));
            }
        }
    }
}

fn add_forward_moves(
    board: &Board,
    moves: &mut Vec<Move>,
    from: u8,
    forward: i8,
    rank27: &Range<u8>,
) {
    let to = (from as i8 + forward) as u8;

    if board.pieces.squares.data[to as usize].is_some() {
        return;
    }

    // promotions
    if to >= 56 || to < 8 {
        moves.push(Move::promote_bishop(from, to));
        moves.push(Move::promote_knight(from, to));
        moves.push(Move::promote_queen(from, to));
        moves.push(Move::promote_rook(from, to));
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
                Move::promote_bishop(10, 2),
                Move::promote_knight(10, 2),
                Move::promote_queen(10, 2),
                Move::promote_rook(10, 2),
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
                Move::promote_bishop(53, 61),
                Move::promote_knight(53, 61),
                Move::promote_queen(53, 61),
                Move::promote_rook(53, 61),
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
        assert_eq!(moves.len(), 56);
    }

    #[test]
    fn it_generates_all_promotion_captures_for_black() {
        let fen = "8/8/8/8/8/8/pppppppp/NNNNNNNN b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 56);
    }

    #[test]
    fn it_generates_captures_for_white_unless_wrong_color() {
        let fen = "4Nnn1/5P2/8/3NNN2/4P3/1NNN4/2P5/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves,
            vec![
                Move::promote_bishop(53, 62),
                Move::promote_knight(53, 62),
                Move::promote_queen(53, 62),
                Move::promote_rook(53, 62),
            ]
        );
    }

    #[test]
    fn it_generates_captures_for_black_unless_wrong_color() {
        let fen = "8/2p5/1nnn4/4p3/3nnn2/8/2p5/1Nnn4 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves,
            vec![
                Move::promote_bishop(10, 1),
                Move::promote_knight(10, 1),
                Move::promote_queen(10, 1),
                Move::promote_rook(10, 1),
            ]
        );
    }
}

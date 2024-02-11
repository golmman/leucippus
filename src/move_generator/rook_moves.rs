use crate::model::board::Board;
use crate::model::r#move::Move;

use super::sliding_moves::add_sliding_moves;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::with_capacity(15);

    for from in &board.pieces.our_rooks {
        let from_x = (from % 8) as i8;
        let from_y = (from / 8) as i8;

        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (0, -1));
        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (-1, 0));
        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (1, 0));
        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (0, 1));
    }

    moves
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_rook_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn it_generates_black_rook_moves_on_an_empty_board() {
        let fen = "8/8/8/2r5/5r2/8/8/8 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 28);
        assert_eq!(
            moves,
            vec![
                Move::from_to(29, 21),
                Move::from_to(29, 13),
                Move::from_to(29, 5),
                Move::from_to(29, 28),
                Move::from_to(29, 27),
                Move::from_to(29, 26),
                Move::from_to(29, 25),
                Move::from_to(29, 24),
                Move::from_to(29, 30),
                Move::from_to(29, 31),
                Move::from_to(29, 37),
                Move::from_to(29, 45),
                Move::from_to(29, 53),
                Move::from_to(29, 61),
                Move::from_to(34, 26),
                Move::from_to(34, 18),
                Move::from_to(34, 10),
                Move::from_to(34, 2),
                Move::from_to(34, 33),
                Move::from_to(34, 32),
                Move::from_to(34, 35),
                Move::from_to(34, 36),
                Move::from_to(34, 37),
                Move::from_to(34, 38),
                Move::from_to(34, 39),
                Move::from_to(34, 42),
                Move::from_to(34, 50),
                Move::from_to(34, 58),
            ]
        );
    }

    #[test]
    fn it_generates_white_rook_moves_on_an_empty_board() {
        let fen = "8/8/8/2R5/5R2/8/8/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 28);
        assert_eq!(
            moves,
            vec![
                Move::from_to(29, 21),
                Move::from_to(29, 13),
                Move::from_to(29, 5),
                Move::from_to(29, 28),
                Move::from_to(29, 27),
                Move::from_to(29, 26),
                Move::from_to(29, 25),
                Move::from_to(29, 24),
                Move::from_to(29, 30),
                Move::from_to(29, 31),
                Move::from_to(29, 37),
                Move::from_to(29, 45),
                Move::from_to(29, 53),
                Move::from_to(29, 61),
                Move::from_to(34, 26),
                Move::from_to(34, 18),
                Move::from_to(34, 10),
                Move::from_to(34, 2),
                Move::from_to(34, 33),
                Move::from_to(34, 32),
                Move::from_to(34, 35),
                Move::from_to(34, 36),
                Move::from_to(34, 37),
                Move::from_to(34, 38),
                Move::from_to(34, 39),
                Move::from_to(34, 42),
                Move::from_to(34, 50),
                Move::from_to(34, 58),
            ]
        );
    }

    #[test]
    fn it_generates_black_rook_capture_moves_on_an_otherwise_empty_board() {
        let fen = "2Q2R2/8/8/Q1r4Q/B4r1P/8/8/2Q2N2 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 28);
        assert_eq!(
            moves,
            vec![
                Move::from_to(29, 21),
                Move::from_to(29, 13),
                Move::from_to(29, 5),
                Move::from_to(29, 28),
                Move::from_to(29, 27),
                Move::from_to(29, 26),
                Move::from_to(29, 25),
                Move::from_to(29, 24),
                Move::from_to(29, 30),
                Move::from_to(29, 31),
                Move::from_to(29, 37),
                Move::from_to(29, 45),
                Move::from_to(29, 53),
                Move::from_to(29, 61),
                Move::from_to(34, 26),
                Move::from_to(34, 18),
                Move::from_to(34, 10),
                Move::from_to(34, 2),
                Move::from_to(34, 33),
                Move::from_to(34, 32),
                Move::from_to(34, 35),
                Move::from_to(34, 36),
                Move::from_to(34, 37),
                Move::from_to(34, 38),
                Move::from_to(34, 39),
                Move::from_to(34, 42),
                Move::from_to(34, 50),
                Move::from_to(34, 58),
            ]
        );
    }

    #[test]
    fn it_generates_white_rook_capture_moves_on_an_otherwise_empty_board() {
        let fen = "2b2n2/8/8/q1R4p/r4R1p/8/8/2b2n2 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 28);
        assert_eq!(
            moves,
            vec![
                Move::from_to(29, 21),
                Move::from_to(29, 13),
                Move::from_to(29, 5),
                Move::from_to(29, 28),
                Move::from_to(29, 27),
                Move::from_to(29, 26),
                Move::from_to(29, 25),
                Move::from_to(29, 24),
                Move::from_to(29, 30),
                Move::from_to(29, 31),
                Move::from_to(29, 37),
                Move::from_to(29, 45),
                Move::from_to(29, 53),
                Move::from_to(29, 61),
                Move::from_to(34, 26),
                Move::from_to(34, 18),
                Move::from_to(34, 10),
                Move::from_to(34, 2),
                Move::from_to(34, 33),
                Move::from_to(34, 32),
                Move::from_to(34, 35),
                Move::from_to(34, 36),
                Move::from_to(34, 37),
                Move::from_to(34, 38),
                Move::from_to(34, 39),
                Move::from_to(34, 42),
                Move::from_to(34, 50),
                Move::from_to(34, 58),
            ]
        );
    }

    #[test]
    fn it_generates_black_rook_moves_in_a_lichess_game() {
        let fen = "2r1r1k1/b4ppp/p7/P2p4/8/3pqPP1/3Q3P/1NR2R1K b - - 5 30";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 16);
        assert_eq!(
            moves,
            vec![
                Move::from_to(58, 50),
                Move::from_to(58, 42),
                Move::from_to(58, 34),
                Move::from_to(58, 26),
                Move::from_to(58, 18),
                Move::from_to(58, 10),
                Move::from_to(58, 2),
                Move::from_to(58, 57),
                Move::from_to(58, 56),
                Move::from_to(58, 59),
                Move::from_to(60, 52),
                Move::from_to(60, 44),
                Move::from_to(60, 36),
                Move::from_to(60, 28),
                Move::from_to(60, 59),
                Move::from_to(60, 61),
            ]
        );
    }

    #[test]
    fn it_generates_white_rook_moves_in_a_lichess_game() {
        let fen = "2r1r1k1/b4ppp/p7/P2p4/8/3p1PP1/3q3P/1NR2R1K w - - 0 31";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 13);
        assert_eq!(
            moves,
            vec![
                Move::from_to(2, 3),
                Move::from_to(2, 4),
                Move::from_to(2, 10),
                Move::from_to(2, 18),
                Move::from_to(2, 26),
                Move::from_to(2, 34),
                Move::from_to(2, 42),
                Move::from_to(2, 50),
                Move::from_to(2, 58),
                Move::from_to(5, 4),
                Move::from_to(5, 3),
                Move::from_to(5, 6),
                Move::from_to(5, 13),
            ]
        );
    }
}

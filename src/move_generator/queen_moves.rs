use crate::model::board::Board;
use crate::model::r#move::Move;

use super::sliding_moves::add_sliding_moves;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for from in &board.pieces.our_queens {
        let from_x = (from % 8) as i8;
        let from_y = (from / 8) as i8;

        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (-1, -1));
        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (1, -1));
        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (-1, 1));
        add_sliding_moves(board, &mut moves, *from, from_x, from_y, (1, 1));
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
    fn it_generates_queen_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn it_generates_black_queen_moves() {
        let fen = "8/8/2b2b2/7b/8/2R2q2/7n/6nq b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 16);
        assert_eq!(
            moves,
            vec![
                Move::from_to(7, 14),
                Move::from_to(21, 12),
                Move::from_to(21, 3),
                Move::from_to(21, 14),
                Move::from_to(21, 28),
                Move::from_to(21, 35),
                Move::from_to(21, 30),
                Move::from_to(21, 13),
                Move::from_to(21, 5),
                Move::from_to(21, 20),
                Move::from_to(21, 19),
                Move::from_to(21, 18),
                Move::from_to(21, 22),
                Move::from_to(21, 23),
                Move::from_to(21, 29),
                Move::from_to(21, 37),
            ]
        );
    }

    #[test]
    fn it_generates_white_queen_moves() {
        let fen = "8/8/2B2R2/7B/8/2r2Q2/7N/6NQ w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 16);
        assert_eq!(
            moves,
            vec![
                Move::from_to(7, 14),
                Move::from_to(21, 12),
                Move::from_to(21, 3),
                Move::from_to(21, 14),
                Move::from_to(21, 28),
                Move::from_to(21, 35),
                Move::from_to(21, 30),
                Move::from_to(21, 13),
                Move::from_to(21, 5),
                Move::from_to(21, 20),
                Move::from_to(21, 19),
                Move::from_to(21, 18),
                Move::from_to(21, 22),
                Move::from_to(21, 23),
                Move::from_to(21, 29),
                Move::from_to(21, 37),
            ]
        );
    }
}

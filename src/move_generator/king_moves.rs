use crate::model::board::Board;
use crate::model::r#move::Move;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for from in &board.pieces.active_kings {
        let from_x = (from % 8) as i32;
        let from_y = (from / 8) as i32;

        check_and_add(board, &mut moves, *from, from_x - 1, from_y - 1);
        check_and_add(board, &mut moves, *from, from_x + 0, from_y - 1);
        check_and_add(board, &mut moves, *from, from_x + 1, from_y - 1);
        check_and_add(board, &mut moves, *from, from_x - 1, from_y + 0);
        check_and_add(board, &mut moves, *from, from_x + 1, from_y + 0);
        check_and_add(board, &mut moves, *from, from_x - 1, from_y + 1);
        check_and_add(board, &mut moves, *from, from_x + 0, from_y + 1);
        check_and_add(board, &mut moves, *from, from_x + 1, from_y + 1);
    }

    moves
}

fn check_and_add(
    board: &Board,
    moves: &mut Vec<Move>,
    from: u8,
    to_x: i32,
    to_y: i32,
) {
    if to_x < 0 || to_x > 7 || to_y < 0 || to_y > 7 {
        return;
    }

    let to = (8 * to_y + to_x) as u8;
    if let Some(piece) = board.pieces.squares.data[to as usize] {
        if piece.get_color() == board.color {
            return;
        }
    }

    moves.push(Move {
        from,
        to,
        special: None,
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_no_king_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn it_generates_one_move_almost_surrounded_by_queens() {
        let fen = "8/8/8/8/8/QQQ5/QKQ5/1QQ5 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 1);

        #[rustfmt::skip]
        assert_eq!(moves[0], Move { from: 9, to: 0, special: None });
    }

    #[test]
    fn it_generates_8_moves_when_alone() {
        let fen = "8/8/8/8/8/8/1K6/8 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 8);

        #[rustfmt::skip]
        assert_eq!(
            moves,
            vec![
                Move { from: 9, to: 0, special: None },
                Move { from: 9, to: 1, special: None },
                Move { from: 9, to: 2, special: None },
                Move { from: 9, to: 8, special: None },
                Move { from: 9, to: 10, special: None },
                Move { from: 9, to: 16, special: None },
                Move { from: 9, to: 17, special: None },
                Move { from: 9, to: 18, special: None },
            ]
        );
    }

    #[test]
    fn it_generates_8_moves_when_surrounded_by_enemies() {
        let fen = "8/8/8/8/8/qqq5/qKq5/qqq5 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 8);

        #[rustfmt::skip]
        assert_eq!(
            moves,
            vec![
                Move { from: 9, to: 0, special: None },
                Move { from: 9, to: 1, special: None },
                Move { from: 9, to: 2, special: None },
                Move { from: 9, to: 8, special: None },
                Move { from: 9, to: 10, special: None },
                Move { from: 9, to: 16, special: None },
                Move { from: 9, to: 17, special: None },
                Move { from: 9, to: 18, special: None },
            ]
        );
    }

    #[test]
    fn it_generates_3_moves_when_in_a_corner() {
        let fen = "8/8/8/8/8/8/8/K7 w - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 3);

        #[rustfmt::skip]
        assert_eq!(
            moves,
            vec![
                Move { from: 0, to: 1, special: None },
                Move { from: 0, to: 8, special: None },
                Move { from: 0, to: 9, special: None },
            ]
        );
    }

    #[test]
    fn it_generates_moves_in_a_complex_position() {
        let fen = "8/8/8/6b1/6Nk/6P1/8/8 b - - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);

        #[rustfmt::skip]
        assert_eq!(
            moves,
            vec![
                Move { from: 31, to: 22, special: None },
                Move { from: 31, to: 23, special: None },
                Move { from: 31, to: 30, special: None },
                Move { from: 31, to: 39, special: None },
            ]
        );
    }
}

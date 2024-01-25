use crate::model::board::Board;
use crate::model::r#move::Move;
use crate::model::types::SquareIndex;

pub fn generate(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for from in &board.pieces.active_knights {
        let from_x = (from % 8) as i32;
        let from_y = (from / 8) as i32;

        add_move(board, &mut moves, *from, from_x - 1, from_y - 2);
        add_move(board, &mut moves, *from, from_x + 1, from_y - 2);
        add_move(board, &mut moves, *from, from_x - 2, from_y - 1);
        add_move(board, &mut moves, *from, from_x + 2, from_y - 1);
        add_move(board, &mut moves, *from, from_x - 2, from_y + 1);
        add_move(board, &mut moves, *from, from_x + 2, from_y + 1);
        add_move(board, &mut moves, *from, from_x - 1, from_y + 2);
        add_move(board, &mut moves, *from, from_x + 1, from_y + 2);
    }

    moves
}

fn add_move(
    board: &Board,
    moves: &mut Vec<Move>,
    from: SquareIndex,
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

    moves.push(Move::from_to(from, to));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_knight_moves_from_the_starting_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves,
            vec![
                Move::from_to(1, 16),
                Move::from_to(1, 18),
                Move::from_to(6, 21),
                Move::from_to(6, 23),
            ]
        );
    }

    #[test]
    fn it_generates_knight_moves_with_4_black_knights() {
        let fen = "rnbqkbnr/pppppppp/8/8/2n5/n7/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let board = Board::from_fen(fen);
        let moves = generate(&board);
        assert_eq!(moves.len(), 14);
        assert_eq!(
            moves,
            vec![
                Move::from_to(16, 1),
                Move::from_to(16, 10),
                Move::from_to(16, 33),
                Move::from_to(26, 9),
                Move::from_to(26, 11),
                Move::from_to(26, 20),
                Move::from_to(26, 32),
                Move::from_to(26, 36),
                Move::from_to(26, 41),
                Move::from_to(26, 43),
                Move::from_to(57, 40),
                Move::from_to(57, 42),
                Move::from_to(62, 45),
                Move::from_to(62, 47),
            ]
        );
    }
}
